#include "IntersectionPath.h"
#include "Ring.h"
#include <cmath>
#include <iostream>

IntersectionPath::IntersectionPath()
    : m_currentPosition(0.f, 0.f), m_previousPosition(0.f, 0.f),
    m_interferenceColor(sf::Color::White), m_energy(0.f), m_isActive(false),
    m_lifetime(0.f), m_maxLifetime(5.0f), m_sourceRing1(nullptr),
    m_sourceRing2(nullptr), m_reflectionIndex1(-1), m_reflectionIndex2(-1),
    m_pulseTimer(0.f), m_baseRadius(3.f)
{
    m_atom.setRadius(m_baseRadius);
    m_atom.setFillColor(m_interferenceColor);
}

void IntersectionPath::initialize(const Ring* ring1, int reflectionIndex1,
    const Ring* ring2, int reflectionIndex2)
{
    m_sourceRing1 = ring1;
    m_sourceRing2 = ring2;
    m_reflectionIndex1 = reflectionIndex1;
    m_reflectionIndex2 = reflectionIndex2;

    // Calculate interference color and energy
    sf::Color color1 = ring1->getColor();
    sf::Color color2 = ring2->getColor();

    // Only create interference if colors are different enough
    const int tolerance = 10;
    if (std::abs(static_cast<int>(color1.r) - static_cast<int>(color2.r)) < tolerance &&
        std::abs(static_cast<int>(color1.g) - static_cast<int>(color2.g)) < tolerance &&
        std::abs(static_cast<int>(color1.b) - static_cast<int>(color2.b)) < tolerance)
    {
        m_isActive = false;
        return; // Colors too similar - destructive interference
    }

    // Additive color mixing
    int r = std::min(255, static_cast<int>(color1.r) + static_cast<int>(color2.r));
    int g = std::min(255, static_cast<int>(color1.g) + static_cast<int>(color2.g));
    int b = std::min(255, static_cast<int>(color1.b) + static_cast<int>(color2.b));

    m_interferenceColor = sf::Color(static_cast<std::uint8_t>(r),
        static_cast<std::uint8_t>(g),
        static_cast<std::uint8_t>(b));

    // Calculate energy based on frequency difference
    float speed1 = Ring::calculateFrequencyBasedSpeed(color1);
    float speed2 = Ring::calculateFrequencyBasedSpeed(color2);
    m_energy = speed1 + speed2 + (std::abs(speed1 - speed2) * 0.5f);

    // Set visual properties based on energy
    m_baseRadius = 2.f + (m_energy * 0.02f);
    m_maxLifetime = 3.f + (m_energy * 0.01f);

    m_atom.setRadius(m_baseRadius);
    m_atom.setFillColor(m_interferenceColor);

    // Try to find initial intersection
    if (updatePosition())
    {
        m_isActive = true;
        m_lifetime = 0.f;
        std::cout << "Intersection path created between reflections "
            << reflectionIndex1 << " and " << reflectionIndex2
            << " - Energy: " << m_energy << std::endl;
    }
}

bool IntersectionPath::updatePosition()
{
    if (!m_sourceRing1 || !m_sourceRing2 || !m_sourceRing1->isAlive() || !m_sourceRing2->isAlive())
    {
        return false;
    }

    // Get the centers and radii for the two shapes we're tracking
    sf::Vector2f center1, center2;
    float radius1 = m_sourceRing1->getRadius();
    float radius2 = m_sourceRing2->getRadius();

    // Determine which shapes to use based on reflection indices
    if (m_reflectionIndex1 == -1)
    {
        // Use main ring for ring1
        center1 = m_sourceRing1->getCenter();
    }
    else
    {
        // Use bounce shape for ring1 - we need access to bounce shapes
        // This is where we'd get the bounce shape center
        // For now, let's use a simplified approach
        center1 = m_sourceRing1->getCenter(); // Placeholder
    }

    if (m_reflectionIndex2 == -1)
    {
        // Use main ring for ring2
        center2 = m_sourceRing2->getCenter();
    }
    else
    {
        // Use bounce shape for ring2
        center2 = m_sourceRing2->getCenter(); // Placeholder
    }

    // Check if circles still intersect
    if (!circlesIntersect(center1, radius1, center2, radius2))
    {
        return false;
    }

    // Calculate new intersection position
    m_previousPosition = m_currentPosition;
    m_currentPosition = calculateIntersection(center1, radius1, center2, radius2);

    return true;
}

void IntersectionPath::update(float deltaTime)
{
    if (!m_isActive) return;

    m_lifetime += deltaTime;
    m_pulseTimer += deltaTime;

    // Update position based on ring states
    if (!updatePosition())
    {
        m_isActive = false;
        return;
    }

    // Check lifetime
    if (m_lifetime >= m_maxLifetime)
    {
        m_isActive = false;
        return;
    }

    // Create pulsing effect based on energy
    float pulseFrequency = 2.0f + (m_energy * 0.05f);
    float pulseIntensity = 0.3f + (m_energy * 0.005f);
    float pulse = std::sin(m_pulseTimer * pulseFrequency) * pulseIntensity + 1.0f;

    // Apply pulsing to visual
    sf::Color pulsingColor = m_interferenceColor;
    pulsingColor.r = static_cast<std::uint8_t>(std::min(255.0f, m_interferenceColor.r * pulse));
    pulsingColor.g = static_cast<std::uint8_t>(std::min(255.0f, m_interferenceColor.g * pulse));
    pulsingColor.b = static_cast<std::uint8_t>(std::min(255.0f, m_interferenceColor.b * pulse));

    // Fade out near end of lifetime
    float lifeRatio = m_lifetime / m_maxLifetime;
    if (lifeRatio > 0.7f)
    {
        float fadeAmount = 1.0f - ((lifeRatio - 0.7f) / 0.3f);
        pulsingColor.a = static_cast<std::uint8_t>(255 * fadeAmount);
    }

    // Update visual properties
    float currentRadius = m_baseRadius * (1.0f + pulse * 0.2f);
    m_atom.setRadius(currentRadius);
    m_atom.setFillColor(pulsingColor);
    m_atom.setPosition(sf::Vector2f(m_currentPosition.x - currentRadius,
        m_currentPosition.y - currentRadius));
}

void IntersectionPath::draw(sf::RenderWindow& window) const
{
    if (m_isActive)
    {
        window.draw(m_atom);
    }
}

sf::Vector2f IntersectionPath::calculateIntersection(sf::Vector2f center1, float radius1,
    sf::Vector2f center2, float radius2)
{
    float dx = center2.x - center1.x;
    float dy = center2.y - center1.y;
    float distance = std::sqrt(dx * dx + dy * dy);

    if (distance == 0 || distance > radius1 + radius2 || distance < std::abs(radius1 - radius2))
    {
        return center1; // Fallback to center1 if no valid intersection
    }

    // Calculate intersection points using circle-circle intersection formula
    float a = (radius1 * radius1 - radius2 * radius2 + distance * distance) / (2.0f * distance);
    float h = std::sqrt(radius1 * radius1 - a * a);

    // Point on line between centers
    sf::Vector2f p;
    p.x = center1.x + (a * dx) / distance;
    p.y = center1.y + (a * dy) / distance;

    // Return one of the intersection points (we could choose which one based on some criteria)
    sf::Vector2f intersection;
    intersection.x = p.x + (h * dy) / distance;
    intersection.y = p.y - (h * dx) / distance;

    return intersection;
}

bool IntersectionPath::circlesIntersect(sf::Vector2f center1, float radius1,
    sf::Vector2f center2, float radius2)
{
    float dx = center2.x - center1.x;
    float dy = center2.y - center1.y;
    float distance = std::sqrt(dx * dx + dy * dy);

    // Circles intersect if distance is less than sum of radii and greater than absolute difference
    return (distance <= radius1 + radius2) && (distance >= std::abs(radius1 - radius2)) && (distance > 0);
}

// PathFollowingAtom implementation
PathFollowingAtom::PathFollowingAtom(IntersectionPath* path)
    : m_path(path), m_pulseTimer(0.f), m_baseRadius(2.f), m_energy(0.f)
{
    if (m_path && m_path->isActive())
    {
        m_baseColor = sf::Color::White; // Will be set by path
        m_shape.setRadius(m_baseRadius);
        m_shape.setFillColor(m_baseColor);
    }
}

void PathFollowingAtom::update(float deltaTime)
{
    if (!isValid()) return;

    m_pulseTimer += deltaTime;

    // Get current position from path
    sf::Vector2f position = m_path->getPosition();

    // Update visual position
    m_shape.setPosition(sf::Vector2f(position.x - m_baseRadius, position.y - m_baseRadius));
}

void PathFollowingAtom::draw(sf::RenderWindow& window) const
{
    if (isValid())
    {
        window.draw(m_shape);
    }
}

bool PathFollowingAtom::isValid() const
{
    return m_path && m_path->isActive();
}