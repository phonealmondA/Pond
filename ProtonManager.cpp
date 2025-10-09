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

    // Remove dead protons
    for (auto& proton : m_protons)
    {
        if (proton && !proton->isAlive())
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
    m_atomCollisionCooldowns.clear();
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
                    m_protons[j].reset(); // Remove absorbed proton
                }
                else
                {
                    m_protons[j]->absorbProton(*m_protons[i]);
                    m_protons[i].reset(); // Remove absorbed proton
                }
            }
        }
    }
}

void ProtonManager::detectAndSpawnFromAtomCollisions(const AtomManager& atomManager)
{
    // NOTE: This is a simplified version since we can't directly access atoms
    // In a full implementation, we'd need to add a friend declaration or public getter to AtomManager
    // For now, this will be a placeholder that we'll implement properly once we integrate with AtomManager

    // TODO: Add atom collision detection once AtomManager provides access
    // The logic should be:
    // 1. Get all high-energy atoms from AtomManager
    // 2. Check distances between all atom pairs
    // 3. If two atoms are close enough (collision), spawn a proton
    // 4. Add cooldown to prevent duplicate spawns
}

bool ProtonManager::isHighEnergyAtom(const PathFollowingAtom& atom) const
{
    return atom.getEnergy() >= MIN_ATOM_ENERGY_THRESHOLD && atom.isAlive();
}

std::vector<const PathFollowingAtom*> ProtonManager::getHighEnergyAtoms(const AtomManager& atomManager) const
{
    // TODO: Implement once AtomManager provides access
    return std::vector<const PathFollowingAtom*>();
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
    for (auto& cooldown : m_atomCollisionCooldowns)
    {
        cooldown.second -= deltaTime;
    }

    // Remove expired cooldowns
    m_atomCollisionCooldowns.erase(
        std::remove_if(m_atomCollisionCooldowns.begin(), m_atomCollisionCooldowns.end(),
            [](const auto& cooldown) { return cooldown.second <= 0.0f; }),
        m_atomCollisionCooldowns.end()
    );
}
