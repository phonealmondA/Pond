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

    // Draw the ring and all bounce reflections
    void draw(sf::RenderWindow& window) const;

    // Check if ring is still alive
    bool isAlive() const;

    // Get current radius
    float getRadius() const;

    // Get center position
    sf::Vector2f getCenter() const;

    // Get current growth speed
    float getGrowthSpeed() const;

    // Get ring color
    sf::Color getColor() const;

    // Set new color (and recalculate speed)
    void setColor(const sf::Color& color);

    // Reset ring to new position
    void reset(sf::Vector2f newCenter);

    // Methods for accessing bounce shapes (needed for intersection detection)
    sf::Vector2f getBounceShapeCenter(int index) const;
    int getBounceShapeCount() const;
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
    size_t getRingCount() const;

    // Get all rings for intersection detection
    std::vector<Ring*> getAllRings() const;

    // Color management methods
    void cycleToNextColor();
    sf::Color getCurrentColor() const;
    std::string getCurrentColorString() const;

    // Get frequency info for current color
    std::string getCurrentFrequencyInfo() const;
};