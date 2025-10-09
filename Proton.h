#pragma once
#include <SFML/Graphics.hpp>

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
    float m_lifetime;
    float m_maxLifetime;

    // Visual effects
    float m_pulseTimer;
    float m_fadeStartTime;

    // Physics parameters
    static constexpr float FRICTION = 0.98f;
    static constexpr float BOUNCE_DAMPENING = 0.7f;
    static constexpr float MIN_RADIUS = 3.0f;
    static constexpr float MAX_RADIUS = 8.0f;

public:
    Proton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy);

    // Update physics and visuals
    void update(float deltaTime, const sf::Vector2u& windowSize);

    // Render to batch renderer
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // Getters
    bool isAlive() const { return m_isAlive; }
    sf::Vector2f getPosition() const { return m_position; }
    sf::Vector2f getVelocity() const { return m_velocity; }
    float getRadius() const { return m_radius; }
    float getEnergy() const { return m_energy; }
    float getMass() const { return m_mass; }
    sf::Color getColor() const { return m_color; }

    // Setters for physics interactions
    void setVelocity(sf::Vector2f velocity) { m_velocity = velocity; }
    void addVelocity(sf::Vector2f deltaVelocity) { m_velocity += deltaVelocity; }

    // Proton interactions
    void absorbProton(const Proton& other);

private:
    // Calculate radius from energy
    float calculateRadius(float energy) const;

    // Calculate mass from energy
    float calculateMass(float energy) const;

    // Handle boundary collisions
    void handleBoundaryCollision(const sf::Vector2u& windowSize);
};
