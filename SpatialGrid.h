#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <unordered_map>

// Forward declaration to avoid circular dependency
struct RingShape;

// Spatial grid for efficient intersection detection
// Reduces O(n²) to approximately O(n) for well-distributed shapes
class SpatialGrid
{
public:
    sf::Vector2u m_windowSize; // Public for easy access

private:
    float m_cellSize;
    std::unordered_map<int, std::vector<const RingShape*>> m_grid;

    // Convert world position to grid cell index
    int getCellIndex(float x, float y) const;

    // Get all neighboring cells (including current cell)
    std::vector<int> getNeighborCells(float x, float y, float radius) const;

public:
    SpatialGrid(const sf::Vector2u& windowSize, float cellSize = 200.0f);

    // Clear and rebuild grid with new shapes
    void rebuild(const std::vector<RingShape>& shapes);

    // Get all shapes that could potentially intersect with given shape
    std::vector<const RingShape*> getPotentialIntersections(const RingShape& shape) const;

    // Get all shape pairs that could potentially intersect (much faster than O(n²))
    std::vector<std::pair<const RingShape*, const RingShape*>> getAllPotentialPairs(
        const std::vector<RingShape>& shapes) const;
};
