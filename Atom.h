#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>

class Ring; // Forward declaration

class Atom
{
private:
    sf::CircleShape m_shape;
    sf::Vector2f m_position;
    sf::Color m_color;
    float m_radius;
    float m_energy; // Energy level based on frequency interference
    float m_lifetime; // How long the atom exists
    float m_maxLifetime;
    bool m_isAlive;

    // Visual pulsing effect based on energy
    float m_pulseTimer;
    float m_baseBrightness;

public:
    // Constructor
    Atom(sf::Vector2f position, sf::Color interferenceColor, float energy);

    // Update the atom (handle lifetime, pulsing effects)
    void update(float deltaTime);

    // Draw the atom
    void draw(sf::RenderWindow& window) const;

    // Check if atom is still alive
    bool isAlive() const;

    // Get position
    sf::Vector2f getPosition() const;

    // Get color
    sf::Color getColor() const;

    // Get energy level
    float getEnergy() const;

    // Static method to calculate interference color between two ring colors
    static sf::Color calculateInterferenceColor(const sf::Color& color1, const sf::Color& color2);

    // Static method to calculate energy from interference
    static float calculateInterferenceEnergy(const sf::Color& color1, const sf::Color& color2);

    // Static method to check if two colors should create interference (not same frequency)
    static bool shouldCreateInterference(const sf::Color& color1, const sf::Color& color2);
};

class AtomManager
{
private:
    std::vector<std::unique_ptr<Atom>> m_atoms;

public:
    AtomManager();

    // Add new atom at intersection point
    void addAtom(sf::Vector2f position, sf::Color color1, sf::Color color2);

    // Update all atoms
    void update(float deltaTime);

    // Draw all atoms
    void draw(sf::RenderWindow& window) const;

    // Clear all atoms
    void clear();

    // Get atom count
    size_t getAtomCount() const;

    // Remove dead atoms
    void removeDeadAtoms();
};