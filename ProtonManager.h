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
    void update(float deltaTime, const sf::Vector2u& windowSize, const AtomManager& atomManager, const std::vector<Ring*>& rings);

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

    // Spawning from high-energy atom collisions
    void detectAndSpawnFromAtomCollisions(const AtomManager& atomManager);

    // Spawn new proton
    void spawnProton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy);

    // Update cooldown timers
    void updateCooldowns(float deltaTime);
};
