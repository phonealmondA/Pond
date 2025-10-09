#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include <unordered_set>
#include <string>
#include "SpatialGrid.h" // Need full definition for unique_ptr
#include "Constants.h"

class Ring; // Forward declaration

// Structure to represent any ring shape (main ring or bounce reflection)
struct RingShape
{
    sf::Vector2f center;
    float radius;
    sf::Color color;
    const Ring* sourceRing;
    int bounceIndex; // -1 for main ring, 0+ for bounce shapes

    // Default constructor
    RingShape() : center(0.f, 0.f), radius(0.f), color(sf::Color::White), sourceRing(nullptr), bounceIndex(-1) {}

    // Parameterized constructor
    RingShape(sf::Vector2f c, float r, sf::Color col, const Ring* ring, int idx = -1)
        : center(c), radius(r), color(col), sourceRing(ring), bounceIndex(idx) {
    }

    // Equality operator for tracking
    bool operator==(const RingShape& other) const {
        return sourceRing == other.sourceRing && bounceIndex == other.bounceIndex;
    }
};

// Path-following atom that moves along intersection points
class PathFollowingAtom
{
private:
    sf::CircleShape m_shape;
    sf::Vector2f m_currentPosition;
    sf::Vector2f m_previousPosition;
    sf::Color m_color;
    float m_radius;
    float m_energy;
    float m_lifetime;
    float m_maxLifetime;
    bool m_isAlive;
    bool m_markedForDeletion;

    // Visual effects
    float m_pulseTimer;
    float m_fadeStartTime;

    // Track which two shapes this atom follows
    RingShape m_shape1;
    RingShape m_shape2;
    bool m_hasValidShapes;

public:
    PathFollowingAtom(const RingShape& shape1, const RingShape& shape2, sf::Vector2f initialPosition);

    // Update position based on current intersection of tracked shapes
    void update(float deltaTime, const std::vector<RingShape>& allCurrentShapes);
    void draw(sf::RenderWindow& window) const;

    // OPTIMIZED: Add to batch renderer
    void addToBatch(class BatchRenderer& batchRenderer) const;

    bool isAlive() const { return m_isAlive && m_hasValidShapes && !m_markedForDeletion; }
    float getLifetime() const { return m_lifetime; }
    sf::Vector2f getPosition() const { return m_currentPosition; }
    float getEnergy() const { return m_energy; }
    void markForDeletion() { m_markedForDeletion = true; }

    // Check if this atom is tracking the given shape pair
    bool isTrackingShapes(const RingShape& shape1, const RingShape& shape2) const;

    // Static utility methods for interference calculations
    static sf::Color calculateInterferenceColor(const sf::Color& color1, const sf::Color& color2);
    static float calculateInterferenceEnergy(const sf::Color& color1, const sf::Color& color2);
    static bool shouldCreateInterference(const sf::Color& color1, const sf::Color& color2);

private:
    // Find current versions of tracked shapes in the current shape list
    bool findCurrentShapes(const std::vector<RingShape>& allCurrentShapes, RingShape& currentShape1, RingShape& currentShape2);

    // Calculate intersection point between two circles
    sf::Vector2f calculateIntersectionPoint(const RingShape& shape1, const RingShape& shape2);

    // Check if two circles intersect
    bool circlesIntersect(const RingShape& shape1, const RingShape& shape2);
};

// Global atom manager with FIFO system and path-following atoms
class AtomManager
{
private:
    // OPTIMIZED: Reduced from 50 to 35 for better performance
    static const size_t MAX_ATOMS = Constants::System::MAX_ATOMS;

    std::vector<std::unique_ptr<PathFollowingAtom>> m_atoms;
    size_t m_nextSlot; // FIFO insertion point
    size_t m_atomCount;

    // Intersection tracking to prevent duplicate atoms for same shape pairs
    std::unordered_set<uint64_t> m_trackedIntersections; // Changed to uint64_t for performance

    // Spatial grid for optimized intersection detection
    std::unique_ptr<SpatialGrid> m_spatialGrid;

    // Helper methods
    uint64_t createIntersectionKey(const RingShape& shape1, const RingShape& shape2) const;
    void cleanupIntersectionTracking(const std::vector<RingShape>& allShapes);

    // Fast distance check using squared distance (avoids sqrt)
    inline bool circlesIntersectFast(const RingShape& shape1, const RingShape& shape2, float& distanceSquared) const;

public:
    AtomManager();

    // Main update method - detects intersections and creates/updates atoms
    void update(float deltaTime, const std::vector<Ring*>& rings, const sf::Vector2u& windowSize);

    // Draw all atoms
    void draw(sf::RenderWindow& window) const;

    // OPTIMIZED: Add all atoms to batch renderer
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // Management methods
    void clear();

    // OPTIMIZED: Inline simple getters
    inline size_t getAtomCount() const { return m_atomCount; }
    inline size_t getMaxAtoms() const { return MAX_ATOMS; }

    // Access to atoms for ProtonManager collision detection
    inline const std::vector<std::unique_ptr<PathFollowingAtom>>& getAtoms() const { return m_atoms; }

private:
    // Intersection detection methods
    void detectNewIntersections(const std::vector<RingShape>& allShapes, const sf::Vector2u& windowSize);
    std::vector<RingShape> getAllShapes(const std::vector<Ring*>& rings) const;
    void checkShapePairForNewIntersection(const RingShape& shape1, const RingShape& shape2, const sf::Vector2u& windowSize);

    // Add new path-following atom
    void addPathFollowingAtom(const RingShape& shape1, const RingShape& shape2, sf::Vector2f intersectionPoint);
};