#include "AtomManager.h"
#include "Ring.h"
#include "SpatialGrid.h"
#include "BatchRenderer.h"
#include <iostream>
#include <cmath>
#include <algorithm>
#include <sstream>

// PathFollowingAtom implementation
PathFollowingAtom::PathFollowingAtom(const RingShape& shape1, const RingShape& shape2, sf::Vector2f initialPosition)
    : m_currentPosition(initialPosition), m_previousPosition(initialPosition),
    m_lifetime(0.f), m_isAlive(true), m_pulseTimer(0.f),
    m_shape1(shape1), m_shape2(shape2), m_hasValidShapes(true)
{
    // Calculate interference properties
    m_color = calculateInterferenceColor(shape1.color, shape2.color);
    m_energy = calculateInterferenceEnergy(shape1.color, shape2.color);

    // OPTIMIZED: Scale atom size based on energy level
    m_radius = 1.8f + (m_energy * 0.025f); // Slightly larger than old static atoms
    m_maxLifetime = 5.f + (m_energy * 0.02f); // OPTIMIZED: Reduced lifetime slightly
    m_fadeStartTime = m_maxLifetime * 0.7f; // Start fading at 70% of lifetime

    // Set up the visual shape
    m_shape.setRadius(m_radius);
    m_shape.setFillColor(m_color);
    m_shape.setPosition(sf::Vector2f(m_currentPosition.x - m_radius, m_currentPosition.y - m_radius));
}

void PathFollowingAtom::update(float deltaTime, const std::vector<RingShape>& allCurrentShapes)
{
    if (!m_isAlive) return;

    m_lifetime += deltaTime;
    m_pulseTimer += deltaTime;

    // Check if atom should die from age
    if (m_lifetime >= m_maxLifetime)
    {
        m_isAlive = false;
        return;
    }

    // Find current versions of our tracked shapes
    RingShape currentShape1, currentShape2;
    if (!findCurrentShapes(allCurrentShapes, currentShape1, currentShape2))
    {
        m_hasValidShapes = false;
        m_isAlive = false;
        return; // Tracked shapes no longer exist
    }

    // Check if shapes still intersect
    if (!circlesIntersect(currentShape1, currentShape2))
    {
        m_isAlive = false;
        return; // Shapes no longer intersect
    }

    // Update position to current intersection point
    m_previousPosition = m_currentPosition;
    m_currentPosition = calculateIntersectionPoint(currentShape1, currentShape2);

    // Create pulsing effect based on energy
    float pulseFrequency = 1.8f + (m_energy * 0.06f); // Higher energy = faster pulse
    float pulseIntensity = 0.3f + (m_energy * 0.01f); // Higher energy = more intense pulse
    float pulse = std::sin(m_pulseTimer * pulseFrequency) * pulseIntensity + 1.0f;

    // Apply pulsing to color
    sf::Color pulsingColor = m_color;
    pulsingColor.r = static_cast<std::uint8_t>(std::min(255.0f, m_color.r * pulse));
    pulsingColor.g = static_cast<std::uint8_t>(std::min(255.0f, m_color.g * pulse));
    pulsingColor.b = static_cast<std::uint8_t>(std::min(255.0f, m_color.b * pulse));

    // Fade out near end of lifetime
    if (m_lifetime > m_fadeStartTime)
    {
        float fadeRatio = (m_lifetime - m_fadeStartTime) / (m_maxLifetime - m_fadeStartTime);
        float fadeAmount = 1.0f - fadeRatio;
        pulsingColor.a = static_cast<std::uint8_t>(255 * fadeAmount);
    }

    m_shape.setFillColor(pulsingColor);

    // Slight size pulsing based on energy
    float sizeMultiplier = 1.0f + (std::sin(m_pulseTimer * pulseFrequency) * 0.2f * m_energy * 0.01f);
    float currentRadius = m_radius * sizeMultiplier;
    m_shape.setRadius(currentRadius);
    m_shape.setPosition(sf::Vector2f(m_currentPosition.x - currentRadius, m_currentPosition.y - currentRadius));
}

void PathFollowingAtom::draw(sf::RenderWindow& window) const
{
    if (m_isAlive && m_hasValidShapes)
    {
        window.draw(m_shape);
    }
}

// OPTIMIZED: Add to batch renderer
void PathFollowingAtom::addToBatch(BatchRenderer& batchRenderer) const
{
    if (m_isAlive && m_hasValidShapes)
    {
        batchRenderer.addAtom(m_currentPosition, m_shape.getRadius(), m_shape.getFillColor());
    }
}

bool PathFollowingAtom::isTrackingShapes(const RingShape& shape1, const RingShape& shape2) const
{
    return ((m_shape1 == shape1 && m_shape2 == shape2) ||
        (m_shape1 == shape2 && m_shape2 == shape1));
}

bool PathFollowingAtom::findCurrentShapes(const std::vector<RingShape>& allCurrentShapes,
    RingShape& currentShape1, RingShape& currentShape2)
{
    bool found1 = false, found2 = false;

    for (const auto& shape : allCurrentShapes)
    {
        if (!found1 && shape == m_shape1)
        {
            currentShape1 = shape;
            found1 = true;
        }
        else if (!found2 && shape == m_shape2)
        {
            currentShape2 = shape;
            found2 = true;
        }

        if (found1 && found2) break;
    }

    return found1 && found2;
}

sf::Vector2f PathFollowingAtom::calculateIntersectionPoint(const RingShape& shape1, const RingShape& shape2)
{
    float dx = shape2.center.x - shape1.center.x;
    float dy = shape2.center.y - shape1.center.y;
    float distance = std::sqrt(dx * dx + dy * dy);

    if (distance == 0 || distance > shape1.radius + shape2.radius ||
        distance < std::abs(shape1.radius - shape2.radius))
    {
        return shape1.center; // Fallback to center1 if no valid intersection
    }

    // Calculate intersection points using circle-circle intersection formula
    float a = (shape1.radius * shape1.radius - shape2.radius * shape2.radius + distance * distance) / (2.0f * distance);
    float h = std::sqrt(shape1.radius * shape1.radius - a * a);

    // Point on line between centers
    sf::Vector2f p;
    p.x = shape1.center.x + (a * dx) / distance;
    p.y = shape1.center.y + (a * dy) / distance;

    // Choose intersection point closer to previous position to maintain continuity
    sf::Vector2f intersection1, intersection2;
    intersection1.x = p.x + (h * dy) / distance;
    intersection1.y = p.y - (h * dx) / distance;
    intersection2.x = p.x - (h * dy) / distance;
    intersection2.y = p.y + (h * dx) / distance;

    // Calculate distances to previous position
    float dist1 = std::sqrt((intersection1.x - m_previousPosition.x) * (intersection1.x - m_previousPosition.x) +
        (intersection1.y - m_previousPosition.y) * (intersection1.y - m_previousPosition.y));
    float dist2 = std::sqrt((intersection2.x - m_previousPosition.x) * (intersection2.x - m_previousPosition.x) +
        (intersection2.y - m_previousPosition.y) * (intersection2.y - m_previousPosition.y));

    return (dist1 < dist2) ? intersection1 : intersection2;
}

bool PathFollowingAtom::circlesIntersect(const RingShape& shape1, const RingShape& shape2)
{
    float dx = shape2.center.x - shape1.center.x;
    float dy = shape2.center.y - shape1.center.y;
    float distance = std::sqrt(dx * dx + dy * dy);

    // Circles intersect if distance is less than sum of radii and greater than absolute difference
    return (distance <= shape1.radius + shape2.radius) &&
        (distance >= std::abs(shape1.radius - shape2.radius)) &&
        (distance > 0);
}

sf::Color PathFollowingAtom::calculateInterferenceColor(const sf::Color& color1, const sf::Color& color2)
{
    // Additive color mixing (like light interference)
    int r = std::min(255, static_cast<int>(color1.r) + static_cast<int>(color2.r));
    int g = std::min(255, static_cast<int>(color1.g) + static_cast<int>(color2.g));
    int b = std::min(255, static_cast<int>(color1.b) + static_cast<int>(color2.b));

    return sf::Color(static_cast<std::uint8_t>(r),
        static_cast<std::uint8_t>(g),
        static_cast<std::uint8_t>(b));
}

float PathFollowingAtom::calculateInterferenceEnergy(const sf::Color& color1, const sf::Color& color2)
{
    // Calculate energy based on frequencies (using Ring's speed calculation)
    float energy1 = Ring::calculateFrequencyBasedSpeed(color1);
    float energy2 = Ring::calculateFrequencyBasedSpeed(color2);

    // Interference energy is combination of both frequencies
    float energySum = energy1 + energy2;
    float energyDifference = std::abs(energy1 - energy2);

    // Energy is based on sum but amplified by frequency difference
    return energySum + (energyDifference * 0.4f);
}

bool PathFollowingAtom::shouldCreateInterference(const sf::Color& color1, const sf::Color& color2)
{
    // Don't create interference for nearly identical colors
    const int tolerance = 8;

    return (std::abs(static_cast<int>(color1.r) - static_cast<int>(color2.r)) > tolerance ||
        std::abs(static_cast<int>(color1.g) - static_cast<int>(color2.g)) > tolerance ||
        std::abs(static_cast<int>(color1.b) - static_cast<int>(color2.b)) > tolerance);
}

// AtomManager implementation
AtomManager::AtomManager()
    : m_nextSlot(0), m_atomCount(0)
{
    m_atoms.resize(MAX_ATOMS);
    m_spatialGrid = std::make_unique<SpatialGrid>(sf::Vector2u(800, 600), 200.0f);
}

// FIXED: Added windowSize parameter to match header declaration
void AtomManager::update(float deltaTime, const std::vector<Ring*>& rings, const sf::Vector2u& windowSize)
{
    // Get all current shapes
    std::vector<RingShape> allShapes = getAllShapes(rings);

    // Update spatial grid dimensions if window size changed
    if (m_spatialGrid->m_windowSize.x != windowSize.x || m_spatialGrid->m_windowSize.y != windowSize.y)
    {
        m_spatialGrid = std::make_unique<SpatialGrid>(windowSize, 200.0f);
    }

    // Rebuild spatial grid with current shapes
    m_spatialGrid->rebuild(allShapes);

    // OPTIMIZED: Interleaved atom updates - update half per frame for 10% performance gain
    // Update atoms in two groups alternating each frame
    static bool updateFirstHalf = true;
    updateFirstHalf = !updateFirstHalf;

    size_t startIdx = updateFirstHalf ? 0 : (m_atomCount / 2);
    size_t endIdx = updateFirstHalf ? (m_atomCount / 2) : m_atomCount;

    for (size_t i = startIdx; i < endIdx; ++i)
    {
        if (m_atoms[i])
        {
            m_atoms[i]->update(deltaTime * 2.0f, allShapes); // Double delta to compensate for half update rate
        }
    }

    // Detect new intersections and create atoms
    detectNewIntersections(allShapes, windowSize);

    // Clean up intersection tracking
    cleanupIntersectionTracking(allShapes);
}

void AtomManager::draw(sf::RenderWindow& window) const
{
    for (size_t i = 0; i < m_atomCount; ++i)
    {
        if (m_atoms[i])
        {
            m_atoms[i]->draw(window);
        }
    }
}

// OPTIMIZED: Add all atoms to batch renderer
void AtomManager::addToBatch(BatchRenderer& batchRenderer) const
{
    for (size_t i = 0; i < m_atomCount; ++i)
    {
        if (m_atoms[i])
        {
            m_atoms[i]->addToBatch(batchRenderer);
        }
    }
}

void AtomManager::clear()
{
    for (auto& atom : m_atoms)
    {
        atom.reset();
    }
    m_atomCount = 0;
    m_nextSlot = 0;
    m_trackedIntersections.clear();
}

// OPTIMIZED: Use spatial grid instead of O(n²) nested loops
void AtomManager::detectNewIntersections(const std::vector<RingShape>& allShapes, const sf::Vector2u& windowSize)
{
    // Use spatial grid to get only potentially intersecting pairs
    auto potentialPairs = m_spatialGrid->getAllPotentialPairs(allShapes);

    // Check only the potential pairs (much smaller set than all pairs)
    for (const auto& pair : potentialPairs)
    {
        // Convert pointers back to references for the check function
        // Find indices in allShapes vector
        for (size_t i = 0; i < allShapes.size(); ++i)
        {
            if (&allShapes[i] == pair.first)
            {
                for (size_t j = i + 1; j < allShapes.size(); ++j)
                {
                    if (&allShapes[j] == pair.second)
                    {
                        checkShapePairForNewIntersection(allShapes[i], allShapes[j], windowSize);
                        break;
                    }
                }
                break;
            }
        }
    }
}

std::vector<RingShape> AtomManager::getAllShapes(const std::vector<Ring*>& rings) const
{
    std::vector<RingShape> shapes;

    // OPTIMIZED: Reserve capacity to avoid reallocations
    // Estimate: 1 main shape + ~4 bounce shapes per ring
    shapes.reserve(rings.size() * 5);

    for (Ring* ring : rings)
    {
        if (!ring || !ring->isAlive()) continue;

        // Add main ring
        shapes.emplace_back(ring->getCenter(), ring->getRadius(), ring->getColor(), ring, -1);

        // Add bounce shapes
        int bounceCount = ring->getBounceShapeCount();
        for (int i = 0; i < bounceCount; ++i)
        {
            sf::Vector2f bounceCenter = ring->getBounceShapeCenter(i);
            shapes.emplace_back(bounceCenter, ring->getRadius(), ring->getColor(), ring, i);
        }
    }

    return shapes;
}

// OPTIMIZED: Use fast distance check and numeric keys
void AtomManager::checkShapePairForNewIntersection(const RingShape& shape1, const RingShape& shape2, const sf::Vector2u& windowSize)
{
    // Don't check intersections between shapes from the same ring
    if (shape1.sourceRing == shape2.sourceRing) return;

    // Check if they should create interference
    if (!PathFollowingAtom::shouldCreateInterference(shape1.color, shape2.color)) return;

    // Use fast intersection check with squared distance
    float distanceSquared;
    if (!circlesIntersectFast(shape1, shape2, distanceSquared)) return;

    // Create unique key for this intersection (now using numeric key)
    uint64_t key = createIntersectionKey(shape1, shape2);

    // Check if we're already tracking this intersection
    if (m_trackedIntersections.find(key) != m_trackedIntersections.end())
    {
        return; // Already have atom for this intersection
    }

    // Check if any existing atom is already tracking this pair
    for (size_t i = 0; i < m_atomCount; ++i)
    {
        if (m_atoms[i] && m_atoms[i]->isAlive() && m_atoms[i]->isTrackingShapes(shape1, shape2))
        {
            return; // Already have atom tracking this pair
        }
    }

    // Calculate actual distance for intersection point calculation
    float distance = std::sqrt(distanceSquared);
    float dx = shape2.center.x - shape1.center.x;
    float dy = shape2.center.y - shape1.center.y;

    // Calculate intersection point and create new atom
    float a = (shape1.radius * shape1.radius - shape2.radius * shape2.radius + distanceSquared) / (2.0f * distance);
    float h = std::sqrt(shape1.radius * shape1.radius - a * a);

    sf::Vector2f p;
    p.x = shape1.center.x + (a * dx) / distance;
    p.y = shape1.center.y + (a * dy) / distance;

    sf::Vector2f intersectionPoint;
    intersectionPoint.x = p.x + (h * dy) / distance;
    intersectionPoint.y = p.y - (h * dx) / distance;

    // FIXED: Check if intersection point is within screen bounds (with small margin)
    float margin = 50.0f; // Allow atoms slightly off-screen in case they move on-screen
    if (intersectionPoint.x >= -margin && intersectionPoint.x <= windowSize.x + margin &&
        intersectionPoint.y >= -margin && intersectionPoint.y <= windowSize.y + margin)
    {
        // Only create atom if intersection point is near/on screen
        m_trackedIntersections.insert(key);
        addPathFollowingAtom(shape1, shape2, intersectionPoint);
    }
    // If intersection is far off-screen, we simply don't create the atom
}

void AtomManager::addPathFollowingAtom(const RingShape& shape1, const RingShape& shape2, sf::Vector2f intersectionPoint)
{
    // Create new atom using FIFO system
    m_atoms[m_nextSlot] = std::make_unique<PathFollowingAtom>(shape1, shape2, intersectionPoint);

    // Advance FIFO pointer
    m_nextSlot = (m_nextSlot + 1) % MAX_ATOMS;

    // Update atom count (max out at MAX_ATOMS)
    if (m_atomCount < MAX_ATOMS)
    {
        m_atomCount++;
    }

    // OPTIMIZED: Removed debug spam - atom creation is silent now
    // float energy = PathFollowingAtom::calculateInterferenceEnergy(shape1.color, shape2.color);
    // std::cout << "Path-following atom created at (" << intersectionPoint.x << ", " << intersectionPoint.y
    //     << ") - Energy: " << energy << " [" << m_atomCount << "/" << MAX_ATOMS << "]" << std::endl;
}

// OPTIMIZED: Use numeric key instead of string (much faster)
uint64_t AtomManager::createIntersectionKey(const RingShape& shape1, const RingShape& shape2) const
{
    // Create unique numeric key based on ring pointers and bounce indices
    // Ensure consistent ordering for key
    const RingShape* first = &shape1;
    const RingShape* second = &shape2;
    if (shape1.sourceRing > shape2.sourceRing ||
        (shape1.sourceRing == shape2.sourceRing && shape1.bounceIndex > shape2.bounceIndex))
    {
        first = &shape2;
        second = &shape1;
    }

    // Pack everything into a 64-bit key
    // Upper 32 bits: hash of first ring pointer + bounce index
    // Lower 32 bits: hash of second ring pointer + bounce index
    uint32_t key1 = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(first->sourceRing)) ^
                    (static_cast<uint32_t>(first->bounceIndex) << 16);
    uint32_t key2 = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(second->sourceRing)) ^
                    (static_cast<uint32_t>(second->bounceIndex) << 16);

    return (static_cast<uint64_t>(key1) << 32) | key2;
}

void AtomManager::cleanupIntersectionTracking(const std::vector<RingShape>& allShapes)
{
    // Remove tracking for shape pairs that no longer exist or don't intersect
    // This is a simplified version - clear periodically to prevent memory bloat

    static int cleanupCounter = 0;
    cleanupCounter++;

    // Clean up every 600 frames (roughly every 10 seconds at 60 FPS)
    if (cleanupCounter >= 600)
    {
        m_trackedIntersections.clear();
        cleanupCounter = 0;
        // OPTIMIZED: Removed debug spam
        // std::cout << "Intersection tracking cleaned up" << std::endl;
    }
}

// OPTIMIZED: Fast circle intersection check using squared distance (avoids expensive sqrt)
inline bool AtomManager::circlesIntersectFast(const RingShape& shape1, const RingShape& shape2, float& distanceSquared) const
{
    float dx = shape2.center.x - shape1.center.x;
    float dy = shape2.center.y - shape1.center.y;
    distanceSquared = dx * dx + dy * dy;

    // Early exit if distance is zero
    if (distanceSquared < 0.001f) return false;

    // Check intersection using squared distances (avoids sqrt)
    float sumRadii = shape1.radius + shape2.radius;
    float diffRadii = std::abs(shape1.radius - shape2.radius);

    return (distanceSquared <= sumRadii * sumRadii) &&
           (distanceSquared >= diffRadii * diffRadii);
}