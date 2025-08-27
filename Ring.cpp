#include "Ring.h"
#include "IntersectionPath.h"
#include <iostream>
#include <algorithm>
#include <sstream>
#include <cmath>
#include <iomanip>

// Calculate growth speed based on light frequency using optimized formula
float Ring::calculateFrequencyBasedSpeed(const sf::Color& color)
{
    // Frequency-based formula: blue dominant = fastest, red dominant = slowest
    // Blue gets highest weight (0.6), green medium (0.3), red lowest (0.1)
    float speedFactor = (color.r * 0.1f + color.g * 0.3f + color.b * 0.6f) / 255.0f;

    // Map to speed range: 20 (slowest) to 120 (fastest) pixels per second
    float minSpeed = 20.0f;
    float maxSpeed = 120.0f;
    float speed = minSpeed + (speedFactor * (maxSpeed - minSpeed));

    return speed;
}

// Ring class implementation with bouncing and frequency-based speed
Ring::Ring(sf::Vector2f center, sf::Color color, float thickness)
    : m_center(center), m_originalCenter(center), m_currentRadius(5.f),
    m_color(color), m_isAlive(true), m_thickness(thickness), m_activePathCount(0)
{
    // Calculate growth speed based on color frequency
    m_growthSpeed = calculateFrequencyBasedSpeed(color);

    // Set up the main visual shape
    m_shape.setRadius(m_currentRadius);
    m_shape.setFillColor(sf::Color::Transparent);
    m_shape.setOutlineThickness(m_thickness);
    m_shape.setOutlineColor(m_color);

    // Position the shape (SFML positions are top-left corner)
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));

    // Initialize intersection paths (they start inactive)
    // No need to explicitly initialize as IntersectionPath constructor handles it
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

    // Update intersection paths
    updateIntersectionPaths(deltaTime);

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

        // Draw intersection paths (atoms)
        for (const auto& path : m_intersectionPaths)
        {
            path.draw(window);
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

float Ring::getGrowthSpeed() const
{
    return m_growthSpeed;
}

sf::Color Ring::getColor() const
{
    return m_color;
}

void Ring::setColor(const sf::Color& color)
{
    m_color = color;
    m_shape.setOutlineColor(color);
    // Recalculate growth speed based on new color
    m_growthSpeed = calculateFrequencyBasedSpeed(color);
}

void Ring::reset(sf::Vector2f newCenter)
{
    m_center = newCenter;
    m_originalCenter = newCenter;
    m_currentRadius = 5.f;
    m_isAlive = true;
    m_bounceData = BounceData(); // Reset bounce data
    m_bounceShapes.clear();

    // Reset intersection paths
    for (auto& path : m_intersectionPaths)
    {
        path.deactivate();
    }
    m_activePathCount = 0;

    m_shape.setRadius(m_currentRadius);
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
    m_shape.setOutlineColor(m_color); // Reset to full opacity

    // Recalculate growth speed based on current color
    m_growthSpeed = calculateFrequencyBasedSpeed(m_color);
}

// New intersection path methods
void Ring::updateIntersectionPaths(float deltaTime)
{
    for (auto& path : m_intersectionPaths)
    {
        if (path.isActive())
        {
            path.update(deltaTime);
        }
    }
}

sf::Vector2f Ring::getBounceShapeCenter(int index) const
{
    if (index == -1)
    {
        return m_center; // Main ring center
    }

    if (index >= 0 && index < static_cast<int>(m_bounceShapes.size()))
    {
        const sf::CircleShape& shape = m_bounceShapes[index];
        sf::Vector2f position = shape.getPosition();
        float radius = shape.getRadius();
        return sf::Vector2f(position.x + radius, position.y + radius); // Convert to center
    }

    return m_center; // Fallback
}

int Ring::getBounceShapeCount() const
{
    return static_cast<int>(m_bounceShapes.size());
}

void Ring::tryCreateIntersectionPath(const Ring& otherRing)
{
    // Only create paths if we have available slots
    if (m_activePathCount >= 6) return;

    // Try to create intersection paths between our reflections and other ring's reflections
    // Priority: main ring vs bounce shapes, then bounce vs bounce

    // Check main ring vs other's bounce shapes
    for (int otherIndex = 0; otherIndex < otherRing.getBounceShapeCount() && m_activePathCount < 6; ++otherIndex)
    {
        // Find an inactive path slot
        for (int pathIndex = 0; pathIndex < 6; ++pathIndex)
        {
            if (!m_intersectionPaths[pathIndex].isActive())
            {
                m_intersectionPaths[pathIndex].initialize(this, -1, &otherRing, otherIndex);
                if (m_intersectionPaths[pathIndex].isActive())
                {
                    m_activePathCount++;
                    break;
                }
            }
        }
    }

    // Check our bounce shapes vs other's main ring
    for (int myIndex = 0; myIndex < getBounceShapeCount() && m_activePathCount < 6; ++myIndex)
    {
        for (int pathIndex = 0; pathIndex < 6; ++pathIndex)
        {
            if (!m_intersectionPaths[pathIndex].isActive())
            {
                m_intersectionPaths[pathIndex].initialize(this, myIndex, &otherRing, -1);
                if (m_intersectionPaths[pathIndex].isActive())
                {
                    m_activePathCount++;
                    break;
                }
            }
        }
    }
}

// RingManager class implementation
RingManager::RingManager()
    : m_randomGen(std::random_device{}()), m_currentColorIndex(0)
{

    // Initialize predefined colors for rings (ordered from lowest to highest frequency/speed)
    m_colors = {
    sf::Color(44, 0, 0),      // Darkest red - slowest
    sf::Color(84, 0, 0),      // Very dark red - extremely slow
    sf::Color(108, 0, 0),     // Very dark red - extremely slow
    sf::Color(138, 0, 0),     // Dark red - very slow  
    sf::Color(162, 0, 0),     // Dark red - very slow  
    sf::Color(182, 0, 0),     // Dark red - very slow  
    sf::Color(192, 0, 0),     // Dark red - very slow  
    sf::Color(212, 0, 0),     // Medium-dark red - slow
    sf::Color(255, 0, 0),     // Red - pure red
    sf::Color(255, 42, 0),    // Red + small green
    sf::Color(255, 84, 0),    // Red + more green (orange-ish)
    sf::Color(255, 128, 0),   // Red + medium green (orange)
    sf::Color(255, 165, 0),   // Orange
    sf::Color(255, 200, 0),   // Yellow-orange
    sf::Color(255, 255, 0),   // Yellow
    sf::Color(200, 255, 0),   // Yellow-green
    sf::Color(128, 255, 0),   // Lime green
    sf::Color(0, 255, 0),     // Green
    sf::Color(0, 255, 84),    // Green + small blue
    sf::Color(0, 255, 128),   // Green + more blue (spring green)
    sf::Color(0, 255, 200),   // Green + high blue (cyan-ish)
    sf::Color(0, 255, 255),   // Cyan
    sf::Color(0, 200, 255),   // Sky blue
    sf::Color(0, 128, 255),   // Light blue
    sf::Color(0, 84, 255),    // Blue + small green
    sf::Color(0, 0, 255),     // Blue
    sf::Color(84, 0, 255),    // Blue + small red (purple)
    sf::Color(128, 0, 255),   // Purple
    sf::Color(200, 0, 255),   // Magenta-purple
    sf::Color(255, 0, 255),   // Magenta
    sf::Color(255, 0, 200),   // Pink-magenta
    sf::Color(255, 84, 255),  // Light magenta
    sf::Color(255, 128, 255), // Light pink
    sf::Color(255, 200, 255), // Very light pink
    sf::Color(255, 255, 255)  // White - fastest
    };

    m_currentColor = m_colors[m_currentColorIndex];
}

void RingManager::addRing(sf::Vector2f position)
{
    m_rings.push_back(std::make_unique<Ring>(position, m_currentColor));
}

void RingManager::detectAndCreateIntersectionPaths()
{
    // Check intersections between all ring pairs for intersection path creation
    for (size_t i = 0; i < m_rings.size(); ++i)
    {
        for (size_t j = i + 1; j < m_rings.size(); ++j)
        {
            Ring* ring1 = m_rings[i].get();
            Ring* ring2 = m_rings[j].get();

            if (!ring1->isAlive() || !ring2->isAlive()) continue;

            // Try to create intersection paths between these rings
            ring1->tryCreateIntersectionPath(*ring2);
            ring2->tryCreateIntersectionPath(*ring1);
        }
    }
}

void RingManager::update(float deltaTime, const sf::Vector2u& windowSize)
{
    // Update all rings
    for (auto& ring : m_rings)
    {
        ring->update(deltaTime, windowSize);
    }

    // Detect and create intersection paths between rings
    detectAndCreateIntersectionPaths();

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
    // Draw rings (which now includes their intersection paths)
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

std::string RingManager::getCurrentFrequencyInfo() const
{
    // Calculate the frequency-based speed for the current color
    float speed = Ring::calculateFrequencyBasedSpeed(m_currentColor);

    std::ostringstream oss;
    oss << getCurrentColorString() << " - Speed: " << std::fixed << std::setprecision(1) << speed << " px/s";

    // Add frequency description
    if (speed < 40.0f) {
        oss << " (Low frequency)";
    }
    else if (speed < 80.0f) {
        oss << " (Medium frequency)";
    }
    else {
        oss << " (High frequency)";
    }

    return oss.str();
}