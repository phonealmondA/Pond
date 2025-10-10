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

    // Update physics for all protons
    updateProtonPhysics(deltaTime, windowSize);

    // Apply charge-based atom forces (attraction/repulsion)
    handleProtonAtomForces(deltaTime, atomManager);

    // Check protons for atom proximity interactions (neutron formation)
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive() && proton->getCharge() == +1)
        {
            bool nearAtom = false;
            sf::Vector2f protonPos = proton->getPosition();

            // Get all atoms from atom manager
            const auto& atoms = atomManager.getAtoms();

            // Check if proton is near any atom
            for (const auto& atom : atoms)
            {
                if (atom && atom->isAlive())
                {
                    sf::Vector2f atomPos = atom->getPosition();
                    float dx = protonPos.x - atomPos.x;
                    float dy = protonPos.y - atomPos.y;
                    float distSquared = dx * dx + dy * dy;

                    // Proton is near atom if distance < NEUTRON_FORMATION_DISTANCE
                    if (distSquared < Constants::ProtonManager::NEUTRON_FORMATION_DISTANCE * Constants::ProtonManager::NEUTRON_FORMATION_DISTANCE)
                    {
                        nearAtom = true;
                        break;
                    }
                }
            }

            // Try neutron formation based on atom proximity
            proton->tryNeutronFormation(deltaTime, nearAtom);
        }
    }

    // Check for electron capture (for neutral protons with neutron)
    for (auto& proton : m_protons)
    {
        if (proton && proton->isAlive() && proton->getCharge() == 0 && proton->getNeutronCount() == 1)
        {
            // Get all atoms from atom manager
            const auto& atoms = atomManager.getAtoms();

            for (const auto& atom : atoms)
            {
                if (atom && atom->isAlive())
                {
                    // Try to capture this electron
                    if (proton->tryCaptureElectron(*atom))
                    {
                        // Electron was captured, mark atom for deletion
                        atom->markForDeletion();
                        break; // One electron per proton
                    }
                }
            }
        }
    }

    // Handle proton-proton interactions
    handleProtonProtonRepulsion(deltaTime);

    // Handle nuclear fusion (MUST happen before absorption to allow He formation)
    handleNuclearFusion(ringManager);

    // Handle absorption (only for non-fusable collisions)
    handleProtonProtonAbsorption();

    // Detect high-energy atom collisions and spawn new protons
    detectAndSpawnFromAtomCollisions(atomManager);

    // Remove dead protons and protons marked for deletion (but preserve stable hydrogen and He4)
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
            if (distance > Constants::ProtonManager::REPULSION_RANGE) continue;

            // Calculate repulsion force (inverse square law)
            float force = Constants::ProtonManager::REPULSION_STRENGTH / (distSquared + Constants::ProtonManager::REPULSION_SAFETY_FACTOR);

            // Normalize direction vector
            if (distance > Constants::Math::EPSILON)
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

        // Skip stable particles - they never get absorbed
        if (m_protons[i]->isStableHydrogen() || m_protons[i]->isStableHelium4()) continue;

        for (size_t j = i + 1; j < m_protons.size(); ++j)
        {
            if (!m_protons[j] || !m_protons[j]->isAlive()) continue;

            // Skip stable particles - they never get absorbed
            if (m_protons[j]->isStableHydrogen() || m_protons[j]->isStableHelium4()) continue;

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

void ProtonManager::handleProtonAtomForces(float deltaTime, const AtomManager& atomManager)
{
    // Get all atoms from atom manager
    const auto& atoms = atomManager.getAtoms();

    // Loop through all protons
    for (auto& proton : m_protons)
    {
        if (!proton || !proton->isAlive()) continue;

        // Skip neutral protons (charge 0)
        int charge = proton->getCharge();
        if (charge == 0) continue;

        sf::Vector2f protonPos = proton->getPosition();

        // Loop through all atoms
        for (const auto& atom : atoms)
        {
            if (!atom || !atom->isAlive()) continue;

            sf::Vector2f atomPos = atom->getPosition();
            sf::Vector2f delta = atomPos - protonPos;
            float distSquared = delta.x * delta.x + delta.y * delta.y;

            // Check if atom is within interaction range
            if (distSquared < Constants::ProtonManager::ATOM_ATTRACTION_RANGE * Constants::ProtonManager::ATOM_ATTRACTION_RANGE && distSquared > Constants::Math::EPSILON)
            {
                float distance = std::sqrt(distSquared);

                // Normalize direction vector
                sf::Vector2f direction = delta / distance;

                // Calculate force using inverse square law
                float force;
                if (charge == +1)
                {
                    // Positive charge: attraction toward atom
                    force = Constants::ProtonManager::ATOM_ATTRACTION_STRENGTH / (distSquared + Constants::ProtonManager::REPULSION_SAFETY_FACTOR);
                }
                else // charge == -1
                {
                    // Negative charge: repulsion away from atom
                    force = -Constants::ProtonManager::ATOM_REPULSION_STRENGTH / (distSquared + Constants::ProtonManager::REPULSION_SAFETY_FACTOR);
                }

                // Apply force as velocity change using proton mass
                float mass = proton->getMass();
                sf::Vector2f acceleration = direction * (force / mass) * deltaTime;
                proton->addVelocity(acceleration);
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
