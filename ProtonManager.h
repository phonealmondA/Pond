#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include "Proton.h"
#include "Constants.h"

// Forward declaration
class PathFollowingAtom;
class AtomManager;
class Ring;
class RingManager;

// ProtonManager - Manages all protons with physics interactions and spawning
class ProtonManager
{
private:
    std::vector<std::unique_ptr<Proton>> m_protons;
    size_t m_nextSlot; // FIFO insertion point

    // Track recent spawn positions to prevent duplicate spawns
    // Store position and remaining cooldown time
    std::vector<std::pair<sf::Vector2f, float>> m_spawnCooldowns;

public:
    ProtonManager();

    // Main update - physics, interactions, and spawning from atoms
    void update(float deltaTime, const sf::Vector2u& windowSize, const AtomManager& atomManager, RingManager& ringManager);

    // Render all protons to batch renderer
    void addToBatch(class BatchRenderer& batchRenderer) const;

    // Management
    void clear();
    size_t getProtonCount() const;

private:
    // Physics updates
    void updateProtonPhysics(float deltaTime, const sf::Vector2u& windowSize);
    void handleProtonProtonRepulsion(float deltaTime);
    void handleProtonProtonAbsorption();
    void handleProtonAtomForces(float deltaTime, const AtomManager& atomManager);
    void handleNuclearFusion(RingManager& ringManager);

    // Spawning from high-energy atom collisions
    void detectAndSpawnFromAtomCollisions(const AtomManager& atomManager);

    // Spawn new proton
    void spawnProton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy, int charge = +1);

    // Update cooldown timers
    void updateCooldowns(float deltaTime);
};
