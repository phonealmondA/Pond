#include "Proton.h"
#include "BatchRenderer.h"
#include "AtomManager.h"
#include <cmath>
#include <algorithm>

Proton::Proton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy)
    : m_position(position)
    , m_velocity(velocity)
    , m_color(color)
    , m_energy(energy)
    , m_isAlive(true)
    , m_markedForDeletion(false)
    , m_lifetime(0.0f)
    , m_pulseTimer(0.0f)
    , m_charge(+1)
    , m_neutronCount(0)
    , m_isStableHydrogen(false)
    , m_waveFieldTimer(0.0f)
{
    m_radius = calculateRadius(energy);
    m_mass = calculateMass(energy);

    // Proton lifetime
    m_maxLifetime = Constants::Proton::DEFAULT_LIFETIME;
    m_fadeStartTime = m_maxLifetime * Constants::Proton::FADE_START_RATIO;
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

    // Friction removed to simulate vacuum
    // m_velocity *= FRICTION;

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

    // Apply charge state visual feedback
    if (m_isStableHydrogen)
    {
        // Stable hydrogen: bright white
        renderColor.r = Constants::Proton::STABLE_HYDROGEN_R;
        renderColor.g = Constants::Proton::STABLE_HYDROGEN_G;
        renderColor.b = Constants::Proton::STABLE_HYDROGEN_B;
        renderRadius *= Constants::Proton::STABLE_HYDROGEN_RADIUS_MULTIPLIER;
    }
    else if (m_charge == 0)
    {
        // Neutral with neutron: gray-white
        renderColor.r = Constants::Proton::NEUTRAL_PROTON_R;
        renderColor.g = Constants::Proton::NEUTRAL_PROTON_G;
        renderColor.b = Constants::Proton::NEUTRAL_PROTON_B;
    }
    else if (m_charge == +1)
    {
        // Bare proton: slight red tint
        renderColor.r = static_cast<std::uint8_t>(std::min(255, static_cast<int>(renderColor.r * Constants::Proton::BARE_PROTON_RED_TINT)));
    }

    // Pulsing effect based on energy
    float pulseFrequency = Constants::Proton::PULSE_FREQUENCY_BASE + (m_energy * Constants::Proton::PULSE_FREQUENCY_ENERGY_FACTOR);
    float pulse = std::sin(m_pulseTimer * pulseFrequency) * Constants::Proton::PULSE_INTENSITY + Constants::Proton::PULSE_BASE;
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
    glowColor1.a = static_cast<std::uint8_t>(glowColor1.a * Constants::Proton::GLOW_LAYER1_ALPHA);
    batchRenderer.addAtom(m_position, renderRadius * Constants::Proton::GLOW_LAYER1_RADIUS, glowColor1);

    sf::Color glowColor2 = renderColor;
    glowColor2.a = static_cast<std::uint8_t>(glowColor2.a * Constants::Proton::GLOW_LAYER2_ALPHA);
    batchRenderer.addAtom(m_position, renderRadius * Constants::Proton::GLOW_LAYER2_RADIUS, glowColor2);
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
    float radius = Constants::Proton::MIN_RADIUS + (energy * Constants::Proton::ENERGY_TO_RADIUS_FACTOR);
    return std::clamp(radius, Constants::Proton::MIN_RADIUS, Constants::Proton::MAX_RADIUS);
}

float Proton::calculateMass(float energy) const
{
    // Mass proportional to energy (E=mc²-ish)
    return energy * Constants::Proton::ENERGY_TO_MASS_FACTOR;
}

void Proton::handleBoundaryCollision(const sf::Vector2u& windowSize)
{
    bool collided = false;

    // Left/right boundaries
    if (m_position.x - m_radius < 0)
    {
        m_position.x = m_radius;
        m_velocity.x = -m_velocity.x * Constants::Proton::BOUNCE_DAMPENING;
        collided = true;
    }
    else if (m_position.x + m_radius > windowSize.x)
    {
        m_position.x = windowSize.x - m_radius;
        m_velocity.x = -m_velocity.x * Constants::Proton::BOUNCE_DAMPENING;
        collided = true;
    }

    // Top/bottom boundaries
    if (m_position.y - m_radius < 0)
    {
        m_position.y = m_radius;
        m_velocity.y = -m_velocity.y * Constants::Proton::BOUNCE_DAMPENING;
        collided = true;
    }
    else if (m_position.y + m_radius > windowSize.y)
    {
        m_position.y = windowSize.y - m_radius;
        m_velocity.y = -m_velocity.y * Constants::Proton::BOUNCE_DAMPENING;
        collided = true;
    }
}

void Proton::tryNeutronFormation(float deltaTime, bool nearAtom)
{
    // Already has neutron, skip
    if (m_charge != +1) return;

    // Not near atom, reset timer
    if (!nearAtom)
    {
        m_waveFieldTimer = 0.0f;
        return;
    }

    // Near atom, accumulate time
    m_waveFieldTimer += deltaTime;

    // Check if neutron formation threshold reached
    if (m_waveFieldTimer >= Constants::Proton::NEUTRON_FORMATION_TIME)
    {
        m_neutronCount = 1;
        m_charge = 0;
        m_radius *= Constants::Proton::NEUTRON_RADIUS_MULTIPLIER;
        m_waveFieldTimer = 0.0f;
    }
}

bool Proton::tryCaptureElectron(const PathFollowingAtom& electron)
{
    // Need to have neutral charge (needs neutron first)
    if (m_charge != 0) return false;

    // Need to have exactly 1 neutron
    if (m_neutronCount != 1) return false;

    // Already stable, skip
    if (m_isStableHydrogen) return false;

    // Calculate distance to electron
    sf::Vector2f delta = electron.getPosition() - m_position;
    float distance = std::sqrt(delta.x * delta.x + delta.y * delta.y);

    // Check if electron is close enough to capture
    if (distance < Constants::Proton::ELECTRON_CAPTURE_DISTANCE)
    {
        m_isStableHydrogen = true;
        m_maxLifetime = Constants::Proton::INFINITE_LIFETIME; // Never die from age
        return true;
    }

    return false;
}
