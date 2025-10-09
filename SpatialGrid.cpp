#include "SpatialGrid.h"
#include "AtomManager.h" // Now we include for the full RingShape definition
#include <cmath>
#include <algorithm>

SpatialGrid::SpatialGrid(const sf::Vector2u& windowSize, float cellSize)
    : m_cellSize(cellSize), m_windowSize(windowSize)
{
}

int SpatialGrid::getCellIndex(float x, float y) const
{
    // Calculate grid coordinates
    int gridX = static_cast<int>(std::floor(x / m_cellSize));
    int gridY = static_cast<int>(std::floor(y / m_cellSize));

    // Calculate grid dimensions
    int gridWidth = static_cast<int>(std::ceil(m_windowSize.x / m_cellSize)) + Constants::SpatialGrid::GRID_MARGIN_CELLS; // +4 for margin

    // Return unique cell index
    return gridY * gridWidth + gridX;
}

std::vector<int> SpatialGrid::getNeighborCells(float x, float y, float radius) const
{
    std::vector<int> cells;

    // Calculate bounding box for the shape
    float minX = x - radius;
    float maxX = x + radius;
    float minY = y - radius;
    float maxY = y + radius;

    // Get cell range
    int minCellX = static_cast<int>(std::floor(minX / m_cellSize));
    int maxCellX = static_cast<int>(std::floor(maxX / m_cellSize));
    int minCellY = static_cast<int>(std::floor(minY / m_cellSize));
    int maxCellY = static_cast<int>(std::floor(maxY / m_cellSize));

    int gridWidth = static_cast<int>(std::ceil(m_windowSize.x / m_cellSize)) + Constants::SpatialGrid::GRID_MARGIN_CELLS;

    // Add all cells in range
    for (int cy = minCellY; cy <= maxCellY; ++cy)
    {
        for (int cx = minCellX; cx <= maxCellX; ++cx)
        {
            cells.push_back(cy * gridWidth + cx);
        }
    }

    return cells;
}

void SpatialGrid::rebuild(const std::vector<RingShape>& shapes)
{
    m_grid.clear();

    // OPTIMIZED: Reserve approximate capacity to reduce allocations
    m_grid.reserve(shapes.size() / 2);

    // Insert each shape into appropriate grid cells
    for (const auto& shape : shapes)
    {
        // OPTIMIZED: Skip shapes that are completely off-screen with margin
        // This dramatically reduces collision checks for off-screen shapes
        if (!isNearViewport(shape, Constants::SpatialGrid::VIEWPORT_MARGIN))
        {
            continue; // Skip this shape entirely - won't participate in collision checks
        }

        // Get all cells this shape overlaps
        std::vector<int> cells = getNeighborCells(shape.center.x, shape.center.y, shape.radius);

        // Add shape to each cell
        for (int cellIndex : cells)
        {
            m_grid[cellIndex].push_back(&shape);
        }
    }
}

std::vector<const RingShape*> SpatialGrid::getPotentialIntersections(const RingShape& shape) const
{
    std::vector<const RingShape*> potentialShapes;

    // OPTIMIZED: Reserve capacity (rough estimate)
    potentialShapes.reserve(Constants::SpatialGrid::POTENTIAL_INTERSECTIONS_RESERVE);

    // Get all cells this shape overlaps
    std::vector<int> cells = getNeighborCells(shape.center.x, shape.center.y, shape.radius);

    // Collect all shapes from those cells
    for (int cellIndex : cells)
    {
        auto it = m_grid.find(cellIndex);
        if (it != m_grid.end())
        {
            for (const RingShape* otherShape : it->second)
            {
                // Skip self
                if (otherShape == &shape) continue;

                // Check if not already added (shapes can be in multiple cells)
                if (std::find(potentialShapes.begin(), potentialShapes.end(), otherShape) == potentialShapes.end())
                {
                    potentialShapes.push_back(otherShape);
                }
            }
        }
    }

    return potentialShapes;
}

std::vector<std::pair<const RingShape*, const RingShape*>> SpatialGrid::getAllPotentialPairs(
    const std::vector<RingShape>& shapes) const
{
    std::vector<std::pair<const RingShape*, const RingShape*>> pairs;

    // OPTIMIZED: Reserve capacity estimate
    pairs.reserve(shapes.size() * 4); // Rough estimate

    // OPTIMIZED: Better early exit - use index-based approach instead of pointer comparison
    for (size_t i = 0; i < shapes.size(); ++i)
    {
        const RingShape& shape1 = shapes[i];
        std::vector<const RingShape*> potentials = getPotentialIntersections(shape1);

        for (const RingShape* shape2Ptr : potentials)
        {
            // OPTIMIZED: Calculate index directly from pointer offset
            ptrdiff_t offset = shape2Ptr - shapes.data();
            if (offset > static_cast<ptrdiff_t>(i) && offset < static_cast<ptrdiff_t>(shapes.size()))
            {
                pairs.emplace_back(&shape1, shape2Ptr);
            }
        }
    }

    // OPTIMIZED: Use move semantics for return
    return std::move(pairs);
}

// OPTIMIZED: Check if shape is near viewport bounds for culling
bool SpatialGrid::isNearViewport(const RingShape& shape, float margin) const
{
    const float windowWidth = static_cast<float>(m_windowSize.x);
    const float windowHeight = static_cast<float>(m_windowSize.y);
    const float cullMargin = shape.radius + margin;

    // Check if shape's bounding box intersects with viewport + margin
    return (shape.center.x + cullMargin >= 0.0f &&
            shape.center.x - cullMargin <= windowWidth &&
            shape.center.y + cullMargin >= 0.0f &&
            shape.center.y - cullMargin <= windowHeight);
}
