#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include <random>

class Ring
{
private:
    struct BounceData
    {
        bool hasBouncedLeft = false;
        bool hasBouncedRight = false;
        bool hasBouncedTop = false;
        bool hasBouncedBottom = false;
        float maxRadius = 0.f; // Track maximum radius reached for bouncing calculations
    };

    sf::CircleShape m_shape;
    sf::Vector2f m_center;
    sf::Vector2f m_originalCenter; // Store original center for bounce calculations
    float m_currentRadius;
    float m_growthSpeed;
    sf::Color m_color;
    bool m_isAlive;
    float m_thickness;
    BounceData m_bounceData;
    std::vector<sf::CircleShape> m_bounceShapes; // Additional shapes for bounce reflections

    // Helper methods for bouncing
    void updateBounceShapes(const sf::Vector2u& windowSize);
    void createBounceShape(sf::Vector2f center, sf::Color color);

public:
    // Calculate growth speed based on color frequency (made public for external use)
    static float calculateFrequencyBasedSpeed(const sf::Color& color);

    // Constructor - calculates speed based on color frequency
    Ring(sf::Vector2f center, sf::Color color = sf::Color::White, float thickness = 3.f);

    // Update the ring (growth and bouncing)
    void update(float deltaTime, const sf::Vector2u& windowSize);

    // Draw the ring and all bounce reflections (individual draw calls - slower)
    void draw(sf::RenderWindow& window) const;

    // OPTIMIZED: Add to batch renderer instead of drawing individually
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // OPTIMIZED: Inline hot path functions to eliminate call overhead
    // Check if ring is still alive
    inline bool isAlive() const { return m_isAlive; }

    // Get current radius
    inline float getRadius() const { return m_currentRadius; }

    // Get center position
    inline sf::Vector2f getCenter() const { return m_center; }

    // Get current growth speed
    inline float getGrowthSpeed() const { return m_growthSpeed; }

    // Get ring color
    inline sf::Color getColor() const { return m_color; }

    // Set new color (and recalculate speed)
    void setColor(const sf::Color& color);

    // Reset ring to new position
    void reset(sf::Vector2f newCenter);

    // OPTIMIZED: Const correctness - methods that don't modify state
    // Methods for accessing bounce shapes (needed for intersection detection)
    sf::Vector2f getBounceShapeCenter(int index) const;
    inline int getBounceShapeCount() const { return static_cast<int>(m_bounceShapes.size()); }
};

class RingManager
{
private:
    std::vector<std::unique_ptr<Ring>> m_rings;
    std::mt19937 m_randomGen;
    std::vector<sf::Color> m_colors;
    sf::Color m_currentColor;
    int m_currentColorIndex;

public:
    RingManager();
    void addRing(sf::Vector2f position);
    void update(float deltaTime, const sf::Vector2u& windowSize);
    void draw(sf::RenderWindow& window) const;
    void clear();

    // OPTIMIZED: Inline simple getter
    inline size_t getRingCount() const { return m_rings.size(); }

    // OPTIMIZED: Batch rendering
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // Get all rings for intersection detection
    std::vector<Ring*> getAllRings() const;

    // Color management methods
    void cycleToNextColor();

    // OPTIMIZED: Const correctness - inline simple getter
    inline sf::Color getCurrentColor() const { return m_currentColor; }

    std::string getCurrentColorString() const;

    // Get frequency info for current color
    std::string getCurrentFrequencyInfo() const;
};