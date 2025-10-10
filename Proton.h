#pragma once
#include <SFML/Graphics.hpp>
#include <string>
#include "Constants.h"

// Proton - A rare, persistent physics particle spawned from high-energy atom collisions
// Protons move independently, repel each other, and can absorb other protons
class Proton
{
private:
    sf::Vector2f m_position;
    sf::Vector2f m_velocity;
    sf::Color m_color;
    float m_energy;
    float m_radius;
    float m_mass;
    bool m_isAlive;
    bool m_markedForDeletion;
    float m_lifetime;
    float m_maxLifetime;

    // Visual effects
    float m_pulseTimer;
    float m_fadeStartTime;

    // Charge state system for hydrogen formation
    int m_charge;
    int m_neutronCount;
    bool m_isStableHydrogen;
    float m_waveFieldTimer;

public:
    Proton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy, int charge = +1);

    // Update physics and visuals
    void update(float deltaTime, const sf::Vector2u& windowSize);

    // Render to batch renderer
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // Getters
    bool isAlive() const { return m_isAlive && !m_markedForDeletion; }
    bool isMarkedForDeletion() const { return m_markedForDeletion; }
    sf::Vector2f getPosition() const { return m_position; }
    sf::Vector2f getVelocity() const { return m_velocity; }
    float getRadius() const { return m_radius; }
    float getEnergy() const { return m_energy; }
    float getMass() const { return m_mass; }
    sf::Color getColor() const { return m_color; }
    int getCharge() const { return m_charge; }
    int getNeutronCount() const { return m_neutronCount; }
    bool isStableHydrogen() const { return m_isStableHydrogen; }
    bool isStableHelium4() const { return m_charge == +2 && m_neutronCount == 2; }

    // Get element label for display
    std::string getElementLabel() const;

    // Setters for physics interactions
    void setVelocity(sf::Vector2f velocity) { m_velocity = velocity; }
    void markForDeletion() { m_markedForDeletion = true; }
    void setNeutronCount(int count) { m_neutronCount = count; }
    void setMaxLifetime(float lifetime) { m_maxLifetime = lifetime; }

    // NOTE: absorbProton() removed - absorption system deleted for performance

    // Charge state mechanics
    void tryNeutronFormation(float deltaTime, bool nearAtom);
    bool tryCaptureElectron(const class PathFollowingAtom& electron);

private:
    // Calculate radius from energy
    float calculateRadius(float energy) const;

    // Calculate mass from energy
    float calculateMass(float energy) const;

    // Handle boundary collisions
    void handleBoundaryCollision(const sf::Vector2u& windowSize);
};
