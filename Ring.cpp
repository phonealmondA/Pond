#include "Ring.h"
#include <iostream>
#include <algorithm>

// Ring class implementation
Ring::Ring(sf::Vector2f center, sf::Color color, float growthSpeed, float thickness)
    : m_center(center), m_currentRadius(5.f), m_growthSpeed(growthSpeed),
    m_color(color), m_isAlive(true), m_thickness(thickness)
{
    // Set up the visual shape
    m_shape.setRadius(m_currentRadius);
    m_shape.setFillColor(sf::Color::Transparent);
    m_shape.setOutlineThickness(m_thickness);
    m_shape.setOutlineColor(m_color);

    // Position the shape (SFML positions are top-left corner) - SFML 3.0 syntax
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
}

void Ring::update(float deltaTime, const sf::Vector2u& windowSize)
{
    if (!m_isAlive) return;

    // Grow the ring
    m_currentRadius += m_growthSpeed * deltaTime;
    m_shape.setRadius(m_currentRadius);

    // Update position to keep centered - SFML 3.0 syntax
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));

    // Check if ring has hit any edge of the screen
    float leftEdge = m_center.x - m_currentRadius;
    float rightEdge = m_center.x + m_currentRadius;
    float topEdge = m_center.y - m_currentRadius;
    float bottomEdge = m_center.y + m_currentRadius;

    if (leftEdge <= 0 || rightEdge >= static_cast<float>(windowSize.x) ||
        topEdge <= 0 || bottomEdge >= static_cast<float>(windowSize.y))
    {
        m_isAlive = false;
    }

    // Optional: Fade out as ring gets bigger (creates nice visual effect)
    if (m_isAlive)
    {
        // Fixed SFML 3.0 type - use std::uint8_t instead of sf::Uint8
        std::uint8_t alpha = static_cast<std::uint8_t>(255 * (1.0f - m_currentRadius / 400.f));
        alpha = std::max(alpha, static_cast<std::uint8_t>(50)); // Minimum visibility
        sf::Color fadedColor = m_color;
        fadedColor.a = alpha;
        m_shape.setOutlineColor(fadedColor);
    }
}

void Ring::draw(sf::RenderWindow& window) const
{
    if (m_isAlive)
    {
        window.draw(m_shape);
    }
}

bool Ring::isAlive() const
{
    return m_isAlive;
}

float Ring::getRadius() const
{
    return m_currentRadius;
}

sf::Vector2f Ring::getCenter() const
{
    return m_center;
}

void Ring::setColor(const sf::Color& color)
{
    m_color = color;
    m_shape.setOutlineColor(color);
}

void Ring::reset(sf::Vector2f newCenter)
{
    m_center = newCenter;
    m_currentRadius = 5.f;
    m_isAlive = true;
    m_shape.setRadius(m_currentRadius);
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
    m_shape.setOutlineColor(m_color); // Reset to full opacity
}

// RingManager class implementation
RingManager::RingManager()
    : m_randomGen(std::random_device{}()), m_colorDist(0, 9)
{
    // Initialize predefined colors for rings
    m_colors = {
        sf::Color::Red, sf::Color::Blue, sf::Color::Green,
        sf::Color::Yellow, sf::Color::Magenta, sf::Color::Cyan,
        sf::Color(255, 165, 0),   // Orange
        sf::Color(128, 0, 128),   // Purple
        sf::Color(255, 192, 203), // Pink
        sf::Color(0, 255, 127)    // Spring Green
    };
}

void RingManager::addRing(sf::Vector2f position)
{
    sf::Color randomColor = m_colors[m_colorDist(m_randomGen)];
    m_rings.push_back(std::make_unique<Ring>(position, randomColor));
}

void RingManager::update(float deltaTime, const sf::Vector2u& windowSize)
{
    // Update all rings
    for (auto& ring : m_rings)
    {
        ring->update(deltaTime, windowSize);
    }

    // Remove dead rings
    m_rings.erase(
        std::remove_if(m_rings.begin(), m_rings.end(),
            [](const std::unique_ptr<Ring>& ring) {
                return !ring->isAlive();
            }),
        m_rings.end()
    );
}

void RingManager::draw(sf::RenderWindow& window) const
{
    for (const auto& ring : m_rings)
    {
        ring->draw(window);
    }
}

void RingManager::clear()
{
    m_rings.clear();
}

size_t RingManager::getRingCount() const
{
    return m_rings.size();
}

void RingManager::addRandomRing(const sf::Vector2u& windowSize)
{
    std::uniform_int_distribution<int> xDist(50, static_cast<int>(windowSize.x) - 50);
    std::uniform_int_distribution<int> yDist(50, static_cast<int>(windowSize.y) - 50);

    sf::Vector2f randomPos(static_cast<float>(xDist(m_randomGen)),
        static_cast<float>(yDist(m_randomGen)));
    addRing(randomPos);
}