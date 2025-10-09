#pragma once
#include <SFML/Graphics.hpp>
#include <vector>
#include <memory>
#include "Proton.h"

// Forward declaration
class PathFollowingAtom;
class AtomManager;

// ProtonManager - Manages all protons with physics interactions and spawning
class ProtonManager
{
private:
    // Maximum protons (much lower than atoms for performance)
    static const size_t MAX_PROTONS = 12;

    std::vector<std::unique_ptr<Proton>> m_protons;
    size_t m_nextSlot; // FIFO insertion point

    // Physics parameters (tunable)
    static constexpr float REPULSION_RANGE = 80.0f;
    static constexpr float REPULSION_STRENGTH = 8000.0f;
    static constexpr float MIN_ATOM_ENERGY_THRESHOLD = 150.0f;
    static constexpr float MIN_COMBINED_ENERGY = 250.0f;

    // Track recent atom collisions to prevent duplicate spawns
    std::vector<std::pair<const PathFollowingAtom*, float>> m_atomCollisionCooldowns; // atom pointer, cooldown timer

public:
    ProtonManager();

    // Main update - physics, interactions, and spawning from atoms
    void update(float deltaTime, const sf::Vector2u& windowSize, const AtomManager& atomManager);

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

    // Spawning from high-energy atom collisions
    void detectAndSpawnFromAtomCollisions(const AtomManager& atomManager);

    // Helper: Check if atom has energy above threshold
    bool isHighEnergyAtom(const PathFollowingAtom& atom) const;

    // Helper: Get atoms from AtomManager (need to access private members)
    std::vector<const PathFollowingAtom*> getHighEnergyAtoms(const AtomManager& atomManager) const;

    // Spawn new proton
    void spawnProton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy);

    // Update cooldown timers
    void updateCooldowns(float deltaTime);
};
