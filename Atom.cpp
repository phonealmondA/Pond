#include "Atom.h"
#include "Ring.h"
#include <iostream>
#include <cmath>
#include <algorithm>

// Atom class implementation
Atom::Atom(sf::Vector2f position, sf::Color interferenceColor, float energy)
    : m_position(position), m_color(interferenceColor), m_energy(energy),
    m_lifetime(0.f), m_isAlive(true), m_pulseTimer(0.f)
{
    // Scale atom size based on energy level
    m_radius = 2.f + (energy * 0.05f); // Base size 2px, grows with energy
    m_maxLifetime = 3.f + (energy * 0.02f); // Higher energy = longer lifetime

    // Set up the visual shape
    m_shape.setRadius(m_radius);
    m_shape.setFillColor(m_color);
    m_shape.setPosition(sf::Vector2f(m_position.x - m_radius, m_position.y - m_radius));

    // Calculate base brightness for pulsing effect
    m_baseBrightness = (m_color.r + m_color.g + m_color.b) / (3.0f * 255.0f);
}

void Atom::update(float deltaTime)
{
    if (!m_isAlive) return;

    m_lifetime += deltaTime;
    m_pulseTimer += deltaTime;

    // Check if atom should die
    if (m_lifetime >= m_maxLifetime)
    {
        m_isAlive = false;
        return;
    }

    // Create pulsing effect based on energy
    float pulseFrequency = 2.0f + (m_energy * 0.1f); // Higher energy = faster pulse
    float pulseIntensity = 0.3f + (m_energy * 0.01f); // Higher energy = more intense pulse
    float pulse = std::sin(m_pulseTimer * pulseFrequency) * pulseIntensity + 1.0f;

    // Apply pulsing to color
    sf::Color pulsingColor = m_color;
    pulsingColor.r = static_cast<std::uint8_t>(std::min(255.0f, m_color.r * pulse));
    pulsingColor.g = static_cast<std::uint8_t>(std::min(255.0f, m_color.g * pulse));
    pulsingColor.b = static_cast<std::uint8_t>(std::min(255.0f, m_color.b * pulse));

    // Fade out near end of lifetime
    float lifeRatio = m_lifetime / m_maxLifetime;
    if (lifeRatio > 0.7f) // Start fading at 70% of lifetime
    {
        float fadeAmount = 1.0f - ((lifeRatio - 0.7f) / 0.3f);
        pulsingColor.a = static_cast<std::uint8_t>(255 * fadeAmount);
    }

    m_shape.setFillColor(pulsingColor);

    // Slight size pulsing based on energy
    float sizeMultiplier = 1.0f + (std::sin(m_pulseTimer * pulseFrequency) * 0.2f * m_energy * 0.01f);
    float currentRadius = m_radius * sizeMultiplier;
    m_shape.setRadius(currentRadius);
    m_shape.setPosition(sf::Vector2f(m_position.x - currentRadius, m_position.y - currentRadius));
}

void Atom::draw(sf::RenderWindow& window) const
{
    if (m_isAlive)
    {
        window.draw(m_shape);
    }
}

bool Atom::isAlive() const
{
    return m_isAlive;
}

sf::Vector2f Atom::getPosition() const
{
    return m_position;
}

sf::Color Atom::getColor() const
{
    return m_color;
}

float Atom::getEnergy() const
{
    return m_energy;
}

sf::Color Atom::calculateInterferenceColor(const sf::Color& color1, const sf::Color& color2)
{
    // Additive color mixing (like light interference)
    int r = std::min(255, static_cast<int>(color1.r) + static_cast<int>(color2.r));
    int g = std::min(255, static_cast<int>(color1.g) + static_cast<int>(color2.g));
    int b = std::min(255, static_cast<int>(color1.b) + static_cast<int>(color2.b));

    return sf::Color(static_cast<std::uint8_t>(r),
        static_cast<std::uint8_t>(g),
        static_cast<std::uint8_t>(b));
}

float Atom::calculateInterferenceEnergy(const sf::Color& color1, const sf::Color& color2)
{
    // Calculate energy based on frequencies (using the same logic as Ring speed)
    float energy1 = Ring::calculateFrequencyBasedSpeed(color1);
    float energy2 = Ring::calculateFrequencyBasedSpeed(color2);

    // Interference energy is combination of both frequencies
    // Higher frequency differences create more energetic interference
    float energySum = energy1 + energy2;
    float energyDifference = std::abs(energy1 - energy2);

    // Energy is based on sum but amplified by frequency difference
    return energySum + (energyDifference * 0.5f);
}

bool Atom::shouldCreateInterference(const sf::Color& color1, const sf::Color& color2)
{
    // Don't create interference for identical colors (perfect wave cancellation)
    const int tolerance = 5; // Allow small differences due to rounding

    return (std::abs(static_cast<int>(color1.r) - static_cast<int>(color2.r)) > tolerance ||
        std::abs(static_cast<int>(color1.g) - static_cast<int>(color2.g)) > tolerance ||
        std::abs(static_cast<int>(color1.b) - static_cast<int>(color2.b)) > tolerance);
}

// AtomManager class implementation
AtomManager::AtomManager()
{
}

void AtomManager::addAtom(sf::Vector2f position, sf::Color color1, sf::Color color2)
{
    // Check if these colors should create interference
    if (!Atom::shouldCreateInterference(color1, color2))
    {
        return; // Same frequency waves cancel out
    }

    // Calculate interference properties
    sf::Color interferenceColor = Atom::calculateInterferenceColor(color1, color2);
    float energy = Atom::calculateInterferenceEnergy(color1, color2);

    // Create new atom
    m_atoms.push_back(std::make_unique<Atom>(position, interferenceColor, energy));

    std::cout << "Atom created at (" << position.x << ", " << position.y
        << ") - Energy: " << energy << std::endl;
}

void AtomManager::update(float deltaTime)
{
    // Update all atoms
    for (auto& atom : m_atoms)
    {
        atom->update(deltaTime);
    }

    // Remove dead atoms
    removeDeadAtoms();
}

void AtomManager::draw(sf::RenderWindow& window) const
{
    for (const auto& atom : m_atoms)
    {
        atom->draw(window);
    }
}

void AtomManager::clear()
{
    m_atoms.clear();
}

size_t AtomManager::getAtomCount() const
{
    return m_atoms.size();
}

void AtomManager::removeDeadAtoms()
{
    m_atoms.erase(
        std::remove_if(m_atoms.begin(), m_atoms.end(),
            [](const std::unique_ptr<Atom>& atom) {
                return !atom->isAlive();
            }),
        m_atoms.end()
    );
}