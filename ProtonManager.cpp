#include "ProtonManager.h"
#include "AtomManager.h"
#include "BatchRenderer.h"
#include <cmath>
#include <random>

ProtonManager::ProtonManager()
    : m_nextSlot(0)
{
    m_protons.resize(MAX_PROTONS);
}

void ProtonManager::update(float deltaTime, const sf::Vector2u& windowSize, const AtomManager& atomManager)
{
    // Update cooldowns
    updateCooldowns(deltaTime);

    // Update physics for all protons
    updateProtonPhysics(deltaTime, windowSize);

    // Handle proton-proton interactions
    handleProtonProtonRepulsion(deltaTime);
    handleProtonProtonAbsorption();

    // Detect high-energy atom collisions and spawn new protons
    detectAndSpawnFromAtomCollisions(atomManager);

    // Remove dead protons and protons marked for deletion
    for (auto& proton : m_protons)
    {
        if (proton && (!proton->isAlive() || proton->isMarkedForDeletion()))
        {
            proton.reset();
        }
    }
}

void ProtonManager::addToBatch(BatchRenderer& batchRenderer) const
{
    for (const auto& proton : m_protons)
    {
        if (proton && proton->isAlive())
        {
            proton->addToBatch(batchRenderer);
        }
    }
}

void ProtonManager::clear()
{
    for (auto& proton : m_protons)
    {
        proton.reset();
    }
    m_nextSlot = 0;
    m_spawnCooldowns.clear();
}

size_t ProtonManager::getProtonCount() const
{
    size_t count = 0;
    for (const auto& proton : m_protons)
    {
        if (proton && proton->isAlive())
        {
            count++;
        }
    }
    return count;
}

void ProtonManager::updateProtonPhysics(float deltaTime, const sf::Vector2u& windowSize)
{
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive())
        {
            proton->update(deltaTime, windowSize);
        }
    }
}

void ProtonManager::handleProtonProtonRepulsion(float deltaTime)
{
    // Check all proton pairs for repulsion
    for (size_t i = 0; i < m_protons.size(); ++i)
    {
        if (!m_protons[i] || !m_protons[i]->isAlive()) continue;

        for (size_t j = i + 1; j < m_protons.size(); ++j)
        {
            if (!m_protons[j] || !m_protons[j]->isAlive()) continue;

            // Calculate distance between protons
            sf::Vector2f pos1 = m_protons[i]->getPosition();
            sf::Vector2f pos2 = m_protons[j]->getPosition();
            sf::Vector2f delta = pos2 - pos1;
            float distSquared = delta.x * delta.x + delta.y * delta.y;
            float distance = std::sqrt(distSquared);

            // Skip if too far apart
            if (distance > REPULSION_RANGE) continue;

            // Calculate repulsion force (inverse square law)
            float force = REPULSION_STRENGTH / (distSquared + 1.0f); // +1 to avoid division by zero

            // Normalize direction vector
            if (distance > 0.001f)
            {
                delta /= distance;

                // Apply force as velocity change (F = ma, assume mass in proton)
                float mass1 = m_protons[i]->getMass();
                float mass2 = m_protons[j]->getMass();

                sf::Vector2f acceleration1 = -delta * (force / mass1) * deltaTime;
                sf::Vector2f acceleration2 = delta * (force / mass2) * deltaTime;

                m_protons[i]->addVelocity(acceleration1);
                m_protons[j]->addVelocity(acceleration2);
            }
        }
    }
}

void ProtonManager::handleProtonProtonAbsorption()
{
    // Check all proton pairs for collision/absorption
    for (size_t i = 0; i < m_protons.size(); ++i)
    {
        if (!m_protons[i] || !m_protons[i]->isAlive()) continue;

        for (size_t j = i + 1; j < m_protons.size(); ++j)
        {
            if (!m_protons[j] || !m_protons[j]->isAlive()) continue;

            // Calculate distance between protons
            sf::Vector2f pos1 = m_protons[i]->getPosition();
            sf::Vector2f pos2 = m_protons[j]->getPosition();
            float dx = pos2.x - pos1.x;
            float dy = pos2.y - pos1.y;
            float distSquared = dx * dx + dy * dy;

            float radius1 = m_protons[i]->getRadius();
            float radius2 = m_protons[j]->getRadius();
            float collisionDist = radius1 + radius2;

            // Check for collision
            if (distSquared < collisionDist * collisionDist)
            {
                // Larger proton absorbs smaller one
                if (m_protons[i]->getEnergy() >= m_protons[j]->getEnergy())
                {
                    m_protons[i]->absorbProton(*m_protons[j]);
                    m_protons[j]->markForDeletion(); // Mark for deletion instead of immediate reset
                }
                else
                {
                    m_protons[j]->absorbProton(*m_protons[i]);
                    m_protons[i]->markForDeletion(); // Mark for deletion instead of immediate reset
                    break; // Exit inner loop since proton i is marked, can't continue with it
                }
            }
        }
    }
}

void ProtonManager::detectAndSpawnFromAtomCollisions(const AtomManager& atomManager)
{
    // Struct to hold safe snapshot of atom data (no pointers)
    struct AtomSnapshot
    {
        sf::Vector2f position;
        float energy;
    };

    // 1. Create safe snapshots of all high-energy atoms (copy data, don't store pointers)
    std::vector<AtomSnapshot> highEnergyAtoms;
    const auto& atoms = atomManager.getAtoms();

    for (const auto& atom : atoms)
    {
        if (atom && atom->isAlive() && atom->getEnergy() >= MIN_ATOM_ENERGY_THRESHOLD)
        {
            AtomSnapshot snapshot;
            snapshot.position = atom->getPosition();
            snapshot.energy = atom->getEnergy();
            highEnergyAtoms.push_back(snapshot);
        }
    }

    // 2. Check distances between all atom snapshot pairs
    for (size_t i = 0; i < highEnergyAtoms.size(); ++i)
    {
        for (size_t j = i + 1; j < highEnergyAtoms.size(); ++j)
        {
            const AtomSnapshot& atom1 = highEnergyAtoms[i];
            const AtomSnapshot& atom2 = highEnergyAtoms[j];

            // 3. Calculate distance between atoms
            float dx = atom2.position.x - atom1.position.x;
            float dy = atom2.position.y - atom1.position.y;
            float distSquared = dx * dx + dy * dy;

            // Collision threshold (atoms are close)
            const float COLLISION_THRESHOLD = 15.0f;
            const float COLLISION_THRESHOLD_SQ = COLLISION_THRESHOLD * COLLISION_THRESHOLD;

            // 4. If atoms collide and have sufficient combined energy, spawn a proton
            if (distSquared < COLLISION_THRESHOLD_SQ)
            {
                float combinedEnergy = atom1.energy + atom2.energy;

                if (combinedEnergy >= MIN_COMBINED_ENERGY)
                {
                    // Calculate spawn position (midpoint between atoms)
                    sf::Vector2f spawnPos = (atom1.position + atom2.position) * 0.5f;

                    // Check if this position is on cooldown
                    bool hasCooldown = false;
                    const float COOLDOWN_DIST = 20.0f; // Prevent spawns within 20 pixels of recent spawns
                    const float COOLDOWN_DIST_SQ = COOLDOWN_DIST * COOLDOWN_DIST;

                    for (const auto& cooldown : m_spawnCooldowns)
                    {
                        float cdx = spawnPos.x - cooldown.first.x;
                        float cdy = spawnPos.y - cooldown.first.y;
                        float cdDistSq = cdx * cdx + cdy * cdy;

                        if (cdDistSq < COOLDOWN_DIST_SQ)
                        {
                            hasCooldown = true;
                            break;
                        }
                    }

                    if (hasCooldown) continue;

                    // Calculate velocity (perpendicular to collision line, based on energy)
                    sf::Vector2f collisionDir(dx, dy);
                    float dist = std::sqrt(distSquared);
                    if (dist > 0.001f) collisionDir /= dist;

                    // Perpendicular direction (rotate 90 degrees)
                    sf::Vector2f perpDir(-collisionDir.y, collisionDir.x);
                    float speed = std::min(combinedEnergy * 0.5f, 200.0f); // Cap max speed
                    sf::Vector2f velocity = perpDir * speed;

                    // Proton color (white for now - could mix atom colors if available)
                    sf::Color protonColor = sf::Color::White;

                    // Spawn the proton
                    spawnProton(spawnPos, velocity, protonColor, combinedEnergy);

                    // 5. Add cooldown to prevent duplicate spawns (0.5 seconds)
                    m_spawnCooldowns.push_back(std::make_pair(spawnPos, 0.5f));
                }
            }
        }
    }
}

void ProtonManager::spawnProton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy)
{
    // Check if we have space
    if (getProtonCount() >= MAX_PROTONS)
    {
        // FIFO replacement - find oldest slot
        m_protons[m_nextSlot] = std::make_unique<Proton>(position, velocity, color, energy);
        m_nextSlot = (m_nextSlot + 1) % MAX_PROTONS;
    }
    else
    {
        // Find first empty slot
        for (size_t i = 0; i < m_protons.size(); ++i)
        {
            if (!m_protons[i] || !m_protons[i]->isAlive())
            {
                m_protons[i] = std::make_unique<Proton>(position, velocity, color, energy);
                m_nextSlot = (i + 1) % MAX_PROTONS;
                break;
            }
        }
    }
}

void ProtonManager::updateCooldowns(float deltaTime)
{
    // Decrease all cooldown timers
    for (auto& cooldown : m_spawnCooldowns)
    {
        cooldown.second -= deltaTime;
    }

    // Remove expired cooldowns
    m_spawnCooldowns.erase(
        std::remove_if(m_spawnCooldowns.begin(), m_spawnCooldowns.end(),
            [](const auto& cooldown) { return cooldown.second <= 0.0f; }),
        m_spawnCooldowns.end()
    );
}
