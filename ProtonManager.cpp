#include "ProtonManager.h"
#include "AtomManager.h"
#include "BatchRenderer.h"
#include "Ring.h"
#include <cmath>
#include <random>
#include <iostream>

ProtonManager::ProtonManager()
    : m_nextSlot(0)
{
    m_protons.resize(Constants::System::MAX_PROTONS);
}

void ProtonManager::update(float deltaTime, const sf::Vector2u& windowSize, const AtomManager& atomManager, RingManager& ringManager)
{
    // Update cooldowns
    updateCooldowns(deltaTime);

    // ===== STEP 1: Simple straight-line physics (NO FORCES!) =====
    updateProtonPhysics(deltaTime, windowSize);

    // ===== STEP 2: Neutron formation (proximity to atoms) =====
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive() && proton->getCharge() == +1)
        {
            bool nearAtom = false;
            sf::Vector2f protonPos = proton->getPosition();
            const auto& atoms = atomManager.getAtoms();

            // Simple distance check - no complex forces
            for (const auto& atom : atoms)
            {
                if (atom && atom->isAlive())
                {
                    sf::Vector2f atomPos = atom->getPosition();
                    float dx = protonPos.x - atomPos.x;
                    float dy = protonPos.y - atomPos.y;
                    float distSquared = dx * dx + dy * dy;

                    if (distSquared < 50.0f * 50.0f)  // 50px proximity threshold
                    {
                        nearAtom = true;
                        break;
                    }
                }
            }

            proton->tryNeutronFormation(deltaTime, nearAtom);
        }
    }

    // ===== STEP 3: Electron capture (for neutral protons) =====
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive() && proton->getCharge() == 0 && proton->getNeutronCount() == 1)
        {
            const auto& atoms = atomManager.getAtoms();

            for (const auto& atom : atoms)
            {
                if (atom && atom->isAlive())
                {
                    if (proton->tryCaptureElectron(*atom))
                    {
                        atom->markForDeletion();
                        break;
                    }
                }
            }
        }
    }

    // ===== STEP 4: Nuclear fusion (only when touching) =====
    handleNuclearFusion(ringManager);

    // ===== STEP 5: Spawn from atom collisions =====
    detectAndSpawnFromAtomCollisions(atomManager);

    // ===== STEP 6: Cleanup dead protons =====
    for (auto& proton : m_protons)
    {
        if (proton && (!proton->isAlive() || proton->isMarkedForDeletion()))
        {
            // Never remove stable hydrogen or stable Helium-4
            if (!proton->isStableHydrogen() && !proton->isStableHelium4())
            {
                proton.reset();
            }
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
        // Only reset unstable protons - stable hydrogen and He4 survive clear
        if (proton && !proton->isStableHydrogen() && !proton->isStableHelium4())
        {
            proton.reset();
        }
    }
    m_nextSlot = 0;
    m_spawnCooldowns.clear();
}

size_t ProtonManager::getProtonCount() const
{
    size_t count = 0;
    for (const auto& proton : m_protons)
    {
        // Only count unstable protons - stable hydrogen and He4 don't count toward MAX_PROTONS limit
        if (proton && proton->isAlive() && !proton->isStableHydrogen() && !proton->isStableHelium4())
        {
            count++;
        }
    }
    return count;
}

void ProtonManager::updateProtonPhysics(float deltaTime, const sf::Vector2u& windowSize)
{
    // SIMPLIFIED: Just update position based on velocity (no forces!)
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive())
        {
            proton->update(deltaTime, windowSize);
        }
    }
}

// ===== DELETED: handleProtonProtonRepulsion() - 2,775 calculations eliminated! =====
// ===== DELETED: handleProtonProtonAbsorption() - Complex absorption removed! =====
// ===== DELETED: handleProtonAtomForces() - 2,625 force calculations eliminated! =====

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
        if (atom && atom->isAlive() && atom->getEnergy() >= Constants::ProtonManager::MIN_ATOM_ENERGY_THRESHOLD)
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
            const float COLLISION_THRESHOLD_SQ = Constants::ProtonManager::COLLISION_THRESHOLD * Constants::ProtonManager::COLLISION_THRESHOLD;

            // 4. If atoms collide and have sufficient combined energy, spawn a proton
            if (distSquared < COLLISION_THRESHOLD_SQ)
            {
                float combinedEnergy = atom1.energy + atom2.energy;

                if (combinedEnergy >= Constants::ProtonManager::MIN_COMBINED_ENERGY)
                {
                    // Calculate spawn position (midpoint between atoms)
                    sf::Vector2f spawnPos = (atom1.position + atom2.position) * 0.5f;

                    // Check if this position is on cooldown
                    bool hasCooldown = false;
                    const float COOLDOWN_DIST_SQ = Constants::ProtonManager::COOLDOWN_DISTANCE * Constants::ProtonManager::COOLDOWN_DISTANCE;

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
                    if (dist > Constants::Math::EPSILON) collisionDir /= dist;

                    // Perpendicular direction (rotate 90 degrees)
                    sf::Vector2f perpDir(-collisionDir.y, collisionDir.x);
                    float speed = std::min(combinedEnergy * Constants::ProtonManager::VELOCITY_ENERGY_FACTOR, Constants::ProtonManager::MAX_SPAWN_SPEED);
                    sf::Vector2f velocity = perpDir * speed;

                    // Proton color (white for now - could mix atom colors if available)
                    sf::Color protonColor = sf::Color::White;

                    // Determine charge based on combined energy
                    int charge = (combinedEnergy >= Constants::ProtonManager::NEGATIVE_PROTON_ENERGY_THRESHOLD) ? -1 : +1;

                    // Spawn the proton
                    spawnProton(spawnPos, velocity, protonColor, combinedEnergy, charge);

                    // 5. Add cooldown to prevent duplicate spawns
                    m_spawnCooldowns.push_back(std::make_pair(spawnPos, Constants::ProtonManager::SPAWN_COOLDOWN_TIME));
                }
            }
        }
    }
}


void ProtonManager::spawnProton(sf::Vector2f position, sf::Vector2f velocity, sf::Color color, float energy, int charge)
{
    // NEW APPROACH: Reject new protons when at capacity (no replacement)
    // Only count unstable protons - stable hydrogen doesn't count toward limit
    if (getProtonCount() >= Constants::System::MAX_PROTONS)
    {
        // At capacity - reject this new proton (don't spawn it)
        return;
    }

    // We have space - find first empty slot and spawn
    for (size_t i = 0; i < m_protons.size(); ++i)
    {
        if (!m_protons[i] || !m_protons[i]->isAlive())
        {
            m_protons[i] = std::make_unique<Proton>(position, velocity, color, energy, charge);

            // Debug output for negative protons
            if (charge == -1)
            {
                std::cout << "Negative proton spawned! Charge: -1, Energy: " << energy << std::endl;
            }

            break;
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

void ProtonManager::handleNuclearFusion(RingManager& ringManager)
{
    // Check all proton pairs for fusion conditions
    for (size_t i = 0; i < m_protons.size(); ++i)
    {
        if (!m_protons[i] || !m_protons[i]->isAlive()) continue;
        if (m_protons[i]->isStableHydrogen() || m_protons[i]->isStableHelium4()) continue;

        for (size_t j = i + 1; j < m_protons.size(); ++j)
        {
            if (!m_protons[j] || !m_protons[j]->isAlive()) continue;
            if (m_protons[j]->isStableHydrogen() || m_protons[j]->isStableHelium4()) continue;

            // Get proton data
            sf::Vector2f pos1 = m_protons[i]->getPosition();
            sf::Vector2f pos2 = m_protons[j]->getPosition();
            sf::Vector2f vel1 = m_protons[i]->getVelocity();
            sf::Vector2f vel2 = m_protons[j]->getVelocity();

            int charge1 = m_protons[i]->getCharge();
            int charge2 = m_protons[j]->getCharge();
            int neutron1 = m_protons[i]->getNeutronCount();
            int neutron2 = m_protons[j]->getNeutronCount();

            // Calculate distance
            float dx = pos2.x - pos1.x;
            float dy = pos2.y - pos1.y;
            float distSquared = dx * dx + dy * dy;

            float radius1 = m_protons[i]->getRadius();
            float radius2 = m_protons[j]->getRadius();
            float collisionDist = radius1 + radius2;

            // Not colliding - skip
            if (distSquared > collisionDist * collisionDist) continue;

            // Calculate relative velocity
            sf::Vector2f relVel = vel1 - vel2;
            float relSpeed = std::sqrt(relVel.x * relVel.x + relVel.y * relVel.y);

            // FUSION CASE 1: Deuterium (0, neutron=1) + Proton (+1, neutron=0) → Helium-3 (+1, neutron=2)
            if ((charge1 == 0 && neutron1 == 1 && charge2 == +1 && neutron2 == 0) ||
                (charge2 == 0 && neutron2 == 1 && charge1 == +1 && neutron1 == 0))
            {
                if (relSpeed > Constants::Proton::DEUTERIUM_FUSION_VELOCITY_THRESHOLD)
                {
                    // Calculate center of mass
                    float mass1 = m_protons[i]->getMass();
                    float mass2 = m_protons[j]->getMass();
                    float totalMass = mass1 + mass2;
                    sf::Vector2f centerOfMass = (pos1 * mass1 + pos2 * mass2) / totalMass;
                    sf::Vector2f combinedVel = (vel1 * mass1 + vel2 * mass2) / totalMass;

                    // Create Helium-3 in first slot
                    float combinedEnergy = m_protons[i]->getEnergy() + m_protons[j]->getEnergy();
                    m_protons[i] = std::make_unique<Proton>(
                        centerOfMass,
                        combinedVel,
                        sf::Color(255, 200, 100),
                        combinedEnergy,
                        +1  // charge
                    );
                    m_protons[i]->setNeutronCount(2);  // Set neutron count to 2

                    // Spawn energy wave at fusion point
                    ringManager.addRing(centerOfMass);

                    // Delete second proton
                    m_protons[j].reset();

                    std::cout << "Helium-3 formed! D + H → He3 + gamma" << std::endl;
                    break;
                }
            }

            // FUSION CASE 2: Helium-3 (+1, neutron=2) + Helium-3 (+1, neutron=2) → Helium-4 (+2, neutron=2) + 2 protons
            else if (charge1 == +1 && neutron1 == 2 && charge2 == +1 && neutron2 == 2)
            {
                if (relSpeed > Constants::Proton::HELIUM3_FUSION_VELOCITY_THRESHOLD)
                {
                    // Calculate center of mass
                    float mass1 = m_protons[i]->getMass();
                    float mass2 = m_protons[j]->getMass();
                    float totalMass = mass1 + mass2;
                    sf::Vector2f centerOfMass = (pos1 * mass1 + pos2 * mass2) / totalMass;
                    sf::Vector2f combinedVel = (vel1 * mass1 + vel2 * mass2) / totalMass;

                    // Create Helium-4 in first slot
                    float combinedEnergy = m_protons[i]->getEnergy() + m_protons[j]->getEnergy();
                    m_protons[i] = std::make_unique<Proton>(
                        centerOfMass,
                        combinedVel,
                        sf::Color(255, 255, 100),
                        combinedEnergy * 0.5f,  // Half energy stays in He4
                        +2  // charge
                    );
                    m_protons[i]->setNeutronCount(2);  // Set neutron count to 2
                    m_protons[i]->setMaxLifetime(-1.0f);  // Helium-4 is stable - never dies

                    // Spawn BIG energy wave at fusion point
                    ringManager.addRing(centerOfMass);
                    ringManager.addRing(centerOfMass);  // Double wave for more energy

                    // Spawn 2 high-energy protons (the released protons)
                    float releaseSpeed = 200.0f;
                    sf::Vector2f perpVel(-relVel.y, relVel.x);  // Perpendicular direction
                    float perpLen = std::sqrt(perpVel.x * perpVel.x + perpVel.y * perpVel.y);
                    if (perpLen > 0.001f) perpVel /= perpLen;

                    spawnProton(centerOfMass + perpVel * 10.0f, perpVel * releaseSpeed, sf::Color::White, combinedEnergy * 0.25f, +1);
                    spawnProton(centerOfMass - perpVel * 10.0f, -perpVel * releaseSpeed, sf::Color::White, combinedEnergy * 0.25f, +1);

                    // Delete second He3
                    m_protons[j].reset();

                    std::cout << "Helium-4 formed! He3 + He3 → He4 + 2H" << std::endl;
                    break;
                }
            }
        }
    }
}

void ProtonManager::drawLabels(sf::RenderWindow& window, const sf::Font& font) const
{
    // SFML 3.0: Text constructor requires font reference
    sf::Text text(font);
    text.setCharacterSize(12);
    text.setFillColor(sf::Color::White);
    text.setOutlineColor(sf::Color::Black);
    text.setOutlineThickness(1.0f);

    for (const auto& proton : m_protons)
    {
        if (proton && proton->isAlive())
        {
            std::string label = proton->getElementLabel();
            text.setString(label);

            // Center text above proton
            sf::FloatRect bounds = text.getLocalBounds();
            sf::Vector2f protonPos = proton->getPosition();
            float protonRadius = proton->getRadius();

            // SFML 3.0: Rect uses .size instead of .width/.height
            text.setOrigin(sf::Vector2f(bounds.size.x / 2.0f, bounds.size.y));
            // SFML 3.0: setPosition takes Vector2f
            text.setPosition(sf::Vector2f(protonPos.x, protonPos.y - protonRadius + 6.0f));

            window.draw(text);
        }
    }
}
