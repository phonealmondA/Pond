#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include <random>
#include <array>
#include "IntersectionPath.h"

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

    // Intersection path management (max 6 paths per ring)
    std::array<IntersectionPath, 6> m_intersectionPaths;
    int m_activePathCount;

    // Helper methods for bouncing
    void updateBounceShapes(const sf::Vector2u& windowSize);
    void createBounceShape(sf::Vector2f center, sf::Color color);

    // Intersection path methods
    void updateIntersectionPaths(float deltaTime);
    void checkForNewIntersections(const Ring& otherRing);

public:
    // Calculate growth speed based on color frequency (made public for RingManager)
    static float calculateFrequencyBasedSpeed(const sf::Color& color);

    // Constructor - now calculates speed based on color frequency
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
};

class RingManager
{
private:
    std::vector<std::unique_ptr<Ring>> m_rings;
    std::mt19937 m_randomGen;
    std::vector<sf::Color> m_colors;
    sf::Color m_currentColor;
    int m_currentColorIndex;

    // Intersection management between rings
    void detectAndCreateIntersectionPaths();

public:
    RingManager();
    void addRing(sf::Vector2f position);
    void update(float deltaTime, const sf::Vector2u& windowSize);
    void draw(sf::RenderWindow& window) const;
    void clear();
    size_t getRingCount() const;

    // Color management methods
    void cycleToNextColor();
    sf::Color getCurrentColor() const;
    std::string getCurrentColorString() const;

    // Get frequency info for current color
    std::string getCurrentFrequencyInfo() const;
};