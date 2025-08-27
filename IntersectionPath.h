#pragma once
#include <SFML/Graphics.hpp>
#include <memory>

class Ring; // Forward declaration

class IntersectionPath
{
private:
    sf::Vector2f m_currentPosition;
    sf::Vector2f m_previousPosition;
    sf::Color m_interferenceColor;
    float m_energy;
    bool m_isActive;
    float m_lifetime;
    float m_maxLifetime;

    // References to the source rings and reflection indices
    const Ring* m_sourceRing1;
    const Ring* m_sourceRing2;
    int m_reflectionIndex1; // which bounce shape from ring1 (-1 for main ring)
    int m_reflectionIndex2; // which bounce shape from ring2 (-1 for main ring)

    // Visual representation
    sf::CircleShape m_atom;
    float m_pulseTimer;
    float m_baseRadius;

public:
    IntersectionPath();

    // Initialize path with two rings and their reflection indices
    void initialize(const Ring* ring1, int reflectionIndex1,
        const Ring* ring2, int reflectionIndex2);

    // Update the path position based on current ring states
    bool updatePosition(); // Returns true if intersection still exists

    // Update visual effects (pulsing, fading, etc.)
    void update(float deltaTime);

    // Draw the atom following this path
    void draw(sf::RenderWindow& window) const;

    // Check if this path is active
    bool isActive() const { return m_isActive; }

    // Get current position
    sf::Vector2f getPosition() const { return m_currentPosition; }

    // Deactivate this path
    void deactivate() { m_isActive = false; }

private:
    // Calculate intersection point between two circles
    sf::Vector2f calculateIntersection(sf::Vector2f center1, float radius1,
        sf::Vector2f center2, float radius2);

    // Check if two circles intersect
    bool circlesIntersect(sf::Vector2f center1, float radius1,
        sf::Vector2f center2, float radius2);
};

class PathFollowingAtom
{
private:
    IntersectionPath* m_path;
    sf::CircleShape m_shape;
    float m_pulseTimer;
    float m_baseRadius;
    sf::Color m_baseColor;
    float m_energy;

public:
    PathFollowingAtom(IntersectionPath* path);

    void update(float deltaTime);
    void draw(sf::RenderWindow& window) const;

    bool isValid() const;
};