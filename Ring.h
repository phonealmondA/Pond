#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include <random>

class Ring
{
private:
    sf::CircleShape m_shape;
    sf::Vector2f m_center;
    float m_currentRadius;
    float m_growthSpeed;
    sf::Color m_color;
    bool m_isAlive;
    float m_thickness;

public:
    // Constructor
    Ring(sf::Vector2f center, sf::Color color = sf::Color::White, float growthSpeed = 60.f, float thickness = 3.f);

    // Update the ring (growth and bounds checking)
    void update(float deltaTime, const sf::Vector2u& windowSize);

    // Draw the ring
    void draw(sf::RenderWindow& window) const;

    // Check if ring is still alive (hasn't hit edges)
    bool isAlive() const;

    // Get current radius (useful for other calculations)
    float getRadius() const;

    // Get center position
    sf::Vector2f getCenter() const;

    // Set new color
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

public:
    RingManager();
    void addRing(sf::Vector2f position);
    void update(float deltaTime, const sf::Vector2u& windowSize);
    void draw(sf::RenderWindow& window) const;
    void clear();
    size_t getRingCount() const;
    //void addRandomRing(const sf::Vector2u& windowSize);

    // New methods for color management
    void cycleToNextColor();
    sf::Color getCurrentColor() const;
    std::string getCurrentColorString() const;
};