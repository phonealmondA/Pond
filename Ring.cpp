#include "Ring.h"
#include "BatchRenderer.h"
#include <iostream>
#include <algorithm>
#include <sstream>
#include <cmath>
#include <iomanip>

// Calculate growth speed based on light frequency using optimized formula
float Ring::calculateFrequencyBasedSpeed(const sf::Color& color)
{
    // Frequency-based formula: blue dominant = fastest, red dominant = slowest
    // Blue gets highest weight, green medium, red lowest
    float speedFactor = (color.r * Constants::Ring::COLOR_WEIGHT_RED +
                         color.g * Constants::Ring::COLOR_WEIGHT_GREEN +
                         color.b * Constants::Ring::COLOR_WEIGHT_BLUE) / Constants::Math::COLOR_MAX;

    // Map to speed range: slowest to fastest pixels per second
    float speed = Constants::Ring::MIN_SPEED + (speedFactor * (Constants::Ring::MAX_SPEED - Constants::Ring::MIN_SPEED));

    return speed;
}

// Ring class implementation with bouncing and frequency-based speed
Ring::Ring(sf::Vector2f center, sf::Color color, float thickness)
    : m_center(center), m_originalCenter(center), m_currentRadius(Constants::Ring::INITIAL_RADIUS),
    m_color(color), m_isAlive(true), m_thickness(thickness)
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

// OPTIMIZED: Aggressive culling - only create bounce shapes that are near screen
void Ring::updateBounceShapes(const sf::Vector2u& windowSize)
{
    m_bounceShapes.clear(); // Clear previous bounce shapes

    // OPTIMIZED: Cache window dimensions as floats to avoid repeated conversions
    const float windowWidth = static_cast<float>(windowSize.x);
    const float windowHeight = static_cast<float>(windowSize.y);

    // Check for collisions and create bounce reflections
    float leftEdge = m_originalCenter.x - m_currentRadius;
    float rightEdge = m_originalCenter.x + m_currentRadius;
    float topEdge = m_originalCenter.y - m_currentRadius;
    float bottomEdge = m_originalCenter.y + m_currentRadius;

    // Track maximum radius for fading effect
    m_bounceData.maxRadius = std::max(m_bounceData.maxRadius, m_currentRadius);

    // OPTIMIZED: Cache bounce color calculation (used multiple times)
    const sf::Color bounceColor(m_color.r, m_color.g, m_color.b,
                                static_cast<std::uint8_t>(m_color.a * Constants::Ring::BOUNCE_REFLECTION_OPACITY));

    // OPTIMIZED: Cache culling margin calculation
    const float cullMargin = m_currentRadius + Constants::Ring::CULL_MARGIN;

    // Helper lambda to check if a bounce shape center would be near the screen
    auto isNearScreen = [&](float x, float y) -> bool {
        return (x + m_currentRadius >= -cullMargin && x - m_currentRadius <= windowWidth + cullMargin &&
                y + m_currentRadius >= -cullMargin && y - m_currentRadius <= windowHeight + cullMargin);
    };

    // Left wall bounce
    if (leftEdge <= 0 && !m_bounceData.hasBouncedLeft)
    {
        m_bounceData.hasBouncedLeft = true;
    }
    if (m_bounceData.hasBouncedLeft)
    {
        // Reflect across left wall (x = 0)
        float reflectedX = -m_originalCenter.x;
        // OPTIMIZED: Only create if near screen
        if (isNearScreen(reflectedX, m_originalCenter.y))
        {
            createBounceShape(sf::Vector2f(reflectedX, m_originalCenter.y), bounceColor);
        }
    }

    // Right wall bounce
    if (rightEdge >= windowWidth && !m_bounceData.hasBouncedRight)
    {
        m_bounceData.hasBouncedRight = true;
    }
    if (m_bounceData.hasBouncedRight)
    {
        // Reflect across right wall
        float reflectedX = Constants::Ring::WINDOW_WIDTH_MULTIPLIER * windowWidth - m_originalCenter.x;
        // OPTIMIZED: Only create if near screen
        if (isNearScreen(reflectedX, m_originalCenter.y))
        {
            createBounceShape(sf::Vector2f(reflectedX, m_originalCenter.y), bounceColor);
        }
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
        // OPTIMIZED: Only create if near screen
        if (isNearScreen(m_originalCenter.x, reflectedY))
        {
            createBounceShape(sf::Vector2f(m_originalCenter.x, reflectedY), bounceColor);
        }
    }

    // Bottom wall bounce
    if (bottomEdge >= windowHeight && !m_bounceData.hasBouncedBottom)
    {
        m_bounceData.hasBouncedBottom = true;
    }
    if (m_bounceData.hasBouncedBottom)
    {
        // Reflect across bottom wall
        float reflectedY = Constants::Ring::WINDOW_HEIGHT_MULTIPLIER * windowHeight - m_originalCenter.y;
        // OPTIMIZED: Only create if near screen
        if (isNearScreen(m_originalCenter.x, reflectedY))
        {
            createBounceShape(sf::Vector2f(m_originalCenter.x, reflectedY), bounceColor);
        }
    }

    // Corner bounces - create diagonal reflections
    // OPTIMIZED: Skip corner bounces entirely - they're rarely visible and expensive
    // Uncomment if you want corner bounces back, but they significantly hurt performance
    /*
    if (m_bounceData.hasBouncedLeft && m_bounceData.hasBouncedTop)
    {
        float reflectedX = -m_originalCenter.x;
        float reflectedY = -m_originalCenter.y;
        if (isNearScreen(reflectedX, reflectedY))
        {
            createBounceShape(sf::Vector2f(reflectedX, reflectedY), bounceColor);
        }
    }
    if (m_bounceData.hasBouncedRight && m_bounceData.hasBouncedTop)
    {
        float reflectedX = 2 * windowWidth - m_originalCenter.x;
        float reflectedY = -m_originalCenter.y;
        if (isNearScreen(reflectedX, reflectedY))
        {
            createBounceShape(sf::Vector2f(reflectedX, reflectedY), bounceColor);
        }
    }
    if (m_bounceData.hasBouncedLeft && m_bounceData.hasBouncedBottom)
    {
        float reflectedX = -m_originalCenter.x;
        float reflectedY = 2 * windowHeight - m_originalCenter.y;
        if (isNearScreen(reflectedX, reflectedY))
        {
            createBounceShape(sf::Vector2f(reflectedX, reflectedY), bounceColor);
        }
    }
    if (m_bounceData.hasBouncedRight && m_bounceData.hasBouncedBottom)
    {
        float reflectedX = 2 * windowWidth - m_originalCenter.x;
        float reflectedY = 2 * windowHeight - m_originalCenter.y;
        if (isNearScreen(reflectedX, reflectedY))
        {
            createBounceShape(sf::Vector2f(reflectedX, reflectedY), bounceColor);
        }
    }
    */

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

    // OPTIMIZED: Kill ring when it gets too large (prevents infinite growth)
    if (m_currentRadius > Constants::Ring::MAX_RADIUS_THRESHOLD)
    {
        m_isAlive = false;
        return; // OPTIMIZED: Early exit
    }

    // OPTIMIZED: Kill ring early if center is far off-screen
    // This prevents off-screen rings from participating in collision detection
    const float windowWidth = static_cast<float>(windowSize.x);
    const float windowHeight = static_cast<float>(windowSize.y);

    if (m_center.x < -Constants::Ring::OFF_SCREEN_MARGIN || m_center.x > windowWidth + Constants::Ring::OFF_SCREEN_MARGIN ||
        m_center.y < -Constants::Ring::OFF_SCREEN_MARGIN || m_center.y > windowHeight + Constants::Ring::OFF_SCREEN_MARGIN)
    {
        m_isAlive = false;
        return; // OPTIMIZED: Early exit for far off-screen rings
    }

    // OPTIMIZED: Cache alpha calculation - fade out as ring gets bigger
    // Calculate alpha only once and reuse
    std::uint8_t alpha = static_cast<std::uint8_t>(Constants::Math::COLOR_MAX * std::max(Constants::Ring::MINIMUM_ALPHA, 1.0f - m_currentRadius / Constants::Ring::ALPHA_CALCULATION_DIVISOR));
    sf::Color fadedColor = m_color;
    fadedColor.a = alpha;
    m_shape.setOutlineColor(fadedColor);
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

// OPTIMIZED: Add to batch renderer (much faster than individual draw calls)
void Ring::addToBatch(BatchRenderer& batchRenderer) const
{
    if (m_isAlive)
    {
        // Add main ring
        batchRenderer.addRing(m_center, m_currentRadius, m_shape.getOutlineColor(), m_thickness);

        // Add all bounce reflections
        for (const auto& bounceShape : m_bounceShapes)
        {
            sf::Vector2f center = bounceShape.getPosition() + sf::Vector2f(bounceShape.getRadius(), bounceShape.getRadius());
            batchRenderer.addRing(center, bounceShape.getRadius(), bounceShape.getOutlineColor(), m_thickness);
        }
    }
}

// OPTIMIZED: These functions are now inlined in the header for performance
// (Definitions removed - see Ring.h)

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
    m_currentRadius = Constants::Ring::RESET_RADIUS;
    m_isAlive = true;
    m_bounceData = BounceData(); // Reset bounce data
    m_bounceShapes.clear();

    m_shape.setRadius(m_currentRadius);
    m_shape.setPosition(sf::Vector2f(m_center.x - m_currentRadius, m_center.y - m_currentRadius));
    m_shape.setOutlineColor(m_color); // Reset to full opacity

    // Recalculate growth speed based on current color
    m_growthSpeed = calculateFrequencyBasedSpeed(m_color);
}

// Methods for accessing bounce shapes
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

// OPTIMIZED: getBounceShapeCount() is now inlined in the header

// RingManager class implementation
RingManager::RingManager()
    : m_randomGen(std::random_device{}()), m_currentColorIndex(0)
{
    // Initialize predefined colors for rings (ordered from lowest to highest frequency/speed)
    m_colors.assign(Constants::RingColors::COLORS, Constants::RingColors::COLORS + Constants::System::COLOR_PALETTE_SIZE);

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
    // Draw rings
    for (const auto& ring : m_rings)
    {
        ring->draw(window);
    }
}

// OPTIMIZED: Batch rendering for all rings
void RingManager::addToBatch(BatchRenderer& batchRenderer) const
{
    for (const auto& ring : m_rings)
    {
        ring->addToBatch(batchRenderer);
    }
}

void RingManager::clear()
{
    m_rings.clear();
}

// OPTIMIZED: getRingCount() is now inlined in the header

std::vector<Ring*> RingManager::getAllRings() const
{
    std::vector<Ring*> rings;

    // OPTIMIZED: Reserve capacity to avoid reallocations
    rings.reserve(m_rings.size());

    for (const auto& ring : m_rings)
    {
        rings.push_back(ring.get());
    }

    // OPTIMIZED: Use move semantics for return (C++11 RVO)
    return std::move(rings);
}

void RingManager::cycleToNextColor()
{
    m_currentColorIndex = (m_currentColorIndex + 1) % m_colors.size();
    m_currentColor = m_colors[m_currentColorIndex];
}

// OPTIMIZED: getCurrentColor() is now inlined in the header

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
    if (speed < Constants::Ring::LOW_FREQUENCY_THRESHOLD) {
        oss << " (Low frequency)";
    }
    else if (speed < Constants::Ring::MEDIUM_FREQUENCY_THRESHOLD) {
        oss << " (Medium frequency)";
    }
    else {
        oss << " (High frequency)";
    }

    return oss.str();
}