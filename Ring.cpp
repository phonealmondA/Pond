#include "Ring.h"
#include <iostream>
#include <algorithm>
#include <sstream>
#include <cmath>

// Ring class implementation with bouncing
Ring::Ring(sf::Vector2f center, sf::Color color, float growthSpeed, float thickness)
    : m_center(center), m_originalCenter(center), m_currentRadius(5.f), m_growthSpeed(growthSpeed),
    m_color(color), m_isAlive(true), m_thickness(thickness)
{
    // Set up the main visual shape
    m_shape.setRadius(m_currentRadius);
    m_shape.setFillColor(sf::Color::Transparent);
    m_shape.setOutlineThickness(m_thickness);
    m_shape.setOutlineColor(m_color);

    // Position the shape (SFML positions are top-left corner)
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
}

void Ring::createBounceShape(sf::Vector2f center, sf::Color color)
{
    sf::CircleShape bounceShape;
    bounceShape.setRadius(m_currentRadius);
    bounceShape.setFillColor(sf::Color::Transparent);
    bounceShape.setOutlineThickness(m_thickness);
    bounceShape.setOutlineColor(color);
    bounceShape.setPosition(sf::Vector2f(center.x - m_currentRadius, center.y - m_currentRadius));
    m_bounceShapes.push_back(bounceShape);
}

void Ring::updateBounceShapes(const sf::Vector2u& windowSize)
{
    m_bounceShapes.clear(); // Clear previous bounce shapes

    float windowWidth = static_cast<float>(windowSize.x);
    float windowHeight = static_cast<float>(windowSize.y);

    // Check for collisions and create bounce reflections
    float leftEdge = m_originalCenter.x - m_currentRadius;
    float rightEdge = m_originalCenter.x + m_currentRadius;
    float topEdge = m_originalCenter.y - m_currentRadius;
    float bottomEdge = m_originalCenter.y + m_currentRadius;

    // Track maximum radius for fading effect
    m_bounceData.maxRadius = std::max(m_bounceData.maxRadius, m_currentRadius);

    // Create a slightly faded color for bounce reflections
    sf::Color bounceColor = m_color;
    bounceColor.a = static_cast<std::uint8_t>(bounceColor.a * 0.7f); // 70% opacity for reflections

    // Left wall bounce
    if (leftEdge <= 0 && !m_bounceData.hasBouncedLeft)
    {
        m_bounceData.hasBouncedLeft = true;
    }
    if (m_bounceData.hasBouncedLeft)
    {
        // Reflect across left wall (x = 0)
        float reflectedX = -m_originalCenter.x;
        createBounceShape(sf::Vector2f(reflectedX, m_originalCenter.y), bounceColor);
    }

    // Right wall bounce
    if (rightEdge >= windowWidth && !m_bounceData.hasBouncedRight)
    {
        m_bounceData.hasBouncedRight = true;
    }
    if (m_bounceData.hasBouncedRight)
    {
        // Reflect across right wall
        float reflectedX = 2 * windowWidth - m_originalCenter.x;
        createBounceShape(sf::Vector2f(reflectedX, m_originalCenter.y), bounceColor);
    }

    // Top wall bounce
    if (topEdge <= 0 && !m_bounceData.hasBouncedTop)
    {
        m_bounceData.hasBouncedTop = true;
    }
    if (m_bounceData.hasBouncedTop)
    {
        // Reflect across top wall (y = 0)
        float reflectedY = -m_originalCenter.y;
        createBounceShape(sf::Vector2f(m_originalCenter.x, reflectedY), bounceColor);
    }

    // Bottom wall bounce
    if (bottomEdge >= windowHeight && !m_bounceData.hasBouncedBottom)
    {
        m_bounceData.hasBouncedBottom = true;
    }
    if (m_bounceData.hasBouncedBottom)
    {
        // Reflect across bottom wall
        float reflectedY = 2 * windowHeight - m_originalCenter.y;
        createBounceShape(sf::Vector2f(m_originalCenter.x, reflectedY), bounceColor);
    }

    // Corner bounces - create diagonal reflections
    if (m_bounceData.hasBouncedLeft && m_bounceData.hasBouncedTop)
    {
        // Top-left corner reflection
        createBounceShape(sf::Vector2f(-m_originalCenter.x, -m_originalCenter.y), bounceColor);
    }
    if (m_bounceData.hasBouncedRight && m_bounceData.hasBouncedTop)
    {
        // Top-right corner reflection
        createBounceShape(sf::Vector2f(2 * windowWidth - m_originalCenter.x, -m_originalCenter.y), bounceColor);
    }
    if (m_bounceData.hasBouncedLeft && m_bounceData.hasBouncedBottom)
    {
        // Bottom-left corner reflection
        createBounceShape(sf::Vector2f(-m_originalCenter.x, 2 * windowHeight - m_originalCenter.y), bounceColor);
    }
    if (m_bounceData.hasBouncedRight && m_bounceData.hasBouncedBottom)
    {
        // Bottom-right corner reflection
        createBounceShape(sf::Vector2f(2 * windowWidth - m_originalCenter.x, 2 * windowHeight - m_originalCenter.y), bounceColor);
    }

    // Update all bounce shape positions and sizes
    for (auto& bounceShape : m_bounceShapes)
    {
        bounceShape.setRadius(m_currentRadius);
        sf::Vector2f bounceCenter = bounceShape.getPosition() + sf::Vector2f(m_currentRadius, m_currentRadius);
        bounceShape.setPosition(sf::Vector2f(bounceCenter.x - m_currentRadius, bounceCenter.y - m_currentRadius));
    }
}

void Ring::update(float deltaTime, const sf::Vector2u& windowSize)
{
    if (!m_isAlive) return;

    // Grow the ring
    m_currentRadius += m_growthSpeed * deltaTime;
    m_shape.setRadius(m_currentRadius);

    // Update position to keep centered
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));

    // Update bounce shapes and reflections
    updateBounceShapes(windowSize);

    // Optional: Kill ring when it gets too large (prevents infinite growth)
    if (m_currentRadius > 2000.f) // Adjust this value as needed
    {
        m_isAlive = false;
    }

    // Optional: Fade out as ring gets bigger for nice visual effect
    if (m_isAlive)
    {
        std::uint8_t alpha = static_cast<std::uint8_t>(255 * std::max(0.1f, 1.0f - m_currentRadius / 800.f));
        sf::Color fadedColor = m_color;
        fadedColor.a = alpha;
        m_shape.setOutlineColor(fadedColor);
    }
}

void Ring::draw(sf::RenderWindow& window) const
{
    if (m_isAlive)
    {
        // Draw main ring
        window.draw(m_shape);

        // Draw all bounce reflections
        for (const auto& bounceShape : m_bounceShapes)
        {
            window.draw(bounceShape);
        }
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
    m_originalCenter = newCenter;
    m_currentRadius = 5.f;
    m_isAlive = true;
    m_bounceData = BounceData(); // Reset bounce data
    m_bounceShapes.clear();

    m_shape.setRadius(m_currentRadius);
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
    m_shape.setOutlineColor(m_color); // Reset to full opacity
}

// RingManager class implementation (unchanged except for cleanup timeout)
RingManager::RingManager()
    : m_randomGen(std::random_device{}()), m_currentColorIndex(0)
{
    // Initialize predefined colors for rings
    m_colors = {
        sf::Color::Red, sf::Color::Blue, sf::Color::Green,
        sf::Color::Yellow, sf::Color::Magenta, sf::Color::Cyan,
        sf::Color(255, 165, 0),   // Orange
        sf::Color(128, 0, 128),   // Purple
        sf::Color(255, 192, 203), // Pink
        sf::Color(0, 255, 127),   // Spring Green
        sf::Color::White,         // White
        sf::Color::Black          // Black
    };

    m_currentColor = m_colors[m_currentColorIndex];
}

void RingManager::addRing(sf::Vector2f position)
{
    m_rings.push_back(std::make_unique<Ring>(position, m_currentColor));
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

void RingManager::cycleToNextColor()
{
    m_currentColorIndex = (m_currentColorIndex + 1) % m_colors.size();
    m_currentColor = m_colors[m_currentColorIndex];
}

sf::Color RingManager::getCurrentColor() const
{
    return m_currentColor;
}

std::string RingManager::getCurrentColorString() const
{
    std::ostringstream oss;
    oss << "RGB(" << static_cast<int>(m_currentColor.r) << ", "
        << static_cast<int>(m_currentColor.g) << ", "
        << static_cast<int>(m_currentColor.b) << ")";
    return oss.str();
}