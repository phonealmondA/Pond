#include "Proton.h"
#include "BatchRenderer.h"
#include <cmath>
#include <algorithm>

Proton::Proton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy)
    : m_position(position)
    , m_velocity(velocity)
    , m_color(color)
    , m_energy(energy)
    , m_isAlive(true)
    , m_lifetime(0.0f)
    , m_pulseTimer(0.0f)
{
    m_radius = calculateRadius(energy);
    m_mass = calculateMass(energy);

    // Proton lifetime: 20 seconds default, or infinite if set to -1
    m_maxLifetime = 20.0f;
    m_fadeStartTime = m_maxLifetime * 0.8f; // Start fading at 80% of lifetime
}

void Proton::update(float deltaTime, const sf::Vector2u& windowSize)
{
    if (!m_isAlive) return;

    // Update lifetime
    m_lifetime += deltaTime;
    m_pulseTimer += deltaTime;

    // Check if proton should die from age (if maxLifetime >= 0)
    if (m_maxLifetime >= 0 && m_lifetime >= m_maxLifetime)
    {
        m_isAlive = false;
        return;
    }

    // Apply friction to velocity
    m_velocity *= FRICTION;

    // Update position based on velocity
    m_position += m_velocity * deltaTime;

    // Handle boundary collisions
    handleBoundaryCollision(windowSize);
}

void Proton::addToBatch(BatchRenderer& batchRenderer) const
{
    if (!m_isAlive) return;

    // Calculate visual properties based on lifetime
    sf::Color renderColor = m_color;
    float renderRadius = m_radius;

    // Pulsing effect based on energy
    float pulseFrequency = 2.0f + (m_energy * 0.01f);
    float pulse = std::sin(m_pulseTimer * pulseFrequency) * 0.2f + 1.0f;
    renderRadius *= pulse;

    // Fade out near end of lifetime
    if (m_maxLifetime >= 0 && m_lifetime > m_fadeStartTime)
    {
        float fadeRatio = (m_lifetime - m_fadeStartTime) / (m_maxLifetime - m_fadeStartTime);
        float fadeAmount = 1.0f - fadeRatio;
        renderColor.a = static_cast<std::uint8_t>(255 * fadeAmount);
    }

    // Render core proton
    batchRenderer.addAtom(m_position, renderRadius, renderColor);

    // Add glow layers for visual polish
    sf::Color glowColor1 = renderColor;
    glowColor1.a = static_cast<std::uint8_t>(glowColor1.a * 0.5f);
    batchRenderer.addAtom(m_position, renderRadius * 1.5f, glowColor1);

    sf::Color glowColor2 = renderColor;
    glowColor2.a = static_cast<std::uint8_t>(glowColor2.a * 0.25f);
    batchRenderer.addAtom(m_position, renderRadius * 2.0f, glowColor2);
}

void Proton::absorbProton(const Proton& other)
{
    // Combine energy
    float totalEnergy = m_energy + other.m_energy;

    // Combine momentum (mass * velocity)
    float totalMass = m_mass + other.m_mass;
    sf::Vector2f totalMomentum = m_velocity * m_mass + other.m_velocity * other.m_mass;

    // Update properties
    m_energy = totalEnergy;
    m_radius = calculateRadius(m_energy);
    m_mass = calculateMass(m_energy);
    m_velocity = totalMomentum / totalMass;

    // Mix colors (weighted by energy)
    float weight1 = m_energy / totalEnergy;
    float weight2 = other.m_energy / totalEnergy;

    m_color.r = static_cast<std::uint8_t>(m_color.r * weight1 + other.m_color.r * weight2);
    m_color.g = static_cast<std::uint8_t>(m_color.g * weight1 + other.m_color.g * weight2);
    m_color.b = static_cast<std::uint8_t>(m_color.b * weight1 + other.m_color.b * weight2);
}

float Proton::calculateRadius(float energy) const
{
    // Scale radius based on energy, clamped to min/max
    float radius = MIN_RADIUS + (energy * 0.01f);
    return std::clamp(radius, MIN_RADIUS, MAX_RADIUS);
}

float Proton::calculateMass(float energy) const
{
    // Mass proportional to energy (E=mc²-ish)
    return energy * 0.1f;
}

void Proton::handleBoundaryCollision(const sf::Vector2u& windowSize)
{
    bool collided = false;

    // Left/right boundaries
    if (m_position.x - m_radius < 0)
    {
        m_position.x = m_radius;
        m_velocity.x = -m_velocity.x * BOUNCE_DAMPENING;
        collided = true;
    }
    else if (m_position.x + m_radius > windowSize.x)
    {
        m_position.x = windowSize.x - m_radius;
        m_velocity.x = -m_velocity.x * BOUNCE_DAMPENING;
        collided = true;
    }

    // Top/bottom boundaries
    if (m_position.y - m_radius < 0)
    {
        m_position.y = m_radius;
        m_velocity.y = -m_velocity.y * BOUNCE_DAMPENING;
        collided = true;
    }
    else if (m_position.y + m_radius > windowSize.y)
    {
        m_position.y = windowSize.y - m_radius;
        m_velocity.y = -m_velocity.y * BOUNCE_DAMPENING;
        collided = true;
    }
}
