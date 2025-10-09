#include <SFML/Graphics.hpp>
#include <iostream>
#include "Ring.h"
#include "AtomManager.h"
#include "BatchRenderer.h"

int main()
{
    // Create a window with 800x600 resolution
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Enhanced Wave Interference with Global Atom System");

    // Ring manager to handle all rings
    RingManager ringManager;

    // Global atom manager with FIFO system (now invisible by default)
    AtomManager atomManager;

    // OPTIMIZED: Batch renderer for performance
    BatchRenderer batchRenderer;

    // Debug mode - shows invisible atoms when enabled
    bool debugShowAtoms = false;

    // Clock for timing
    sf::Clock clock;
    sf::Clock infoTimer; // For periodic info display

    // Create an initial center ring to demonstrate bouncing
    ringManager.addRing(sf::Vector2f(400.f, 300.f)); // Center ring

    std::cout << "=== ENHANCED WAVE INTERFERENCE WITH GLOBAL ATOM SYSTEM ===" << std::endl;
    std::cout << "Controls:" << std::endl;
    std::cout << "- Left click: Create ring at cursor position" << std::endl;
    std::cout << "- Right click: Change ring color/frequency" << std::endl;
    std::cout << "- D: Toggle debug atom visualization" << std::endl;
    std::cout << "- Space: Clear all rings and atoms" << std::endl;
    std::cout << "- Escape: Exit" << std::endl;
    std::cout << std::endl;
    std::cout << "Wave Physics:" << std::endl;
    std::cout << "- Ring speed is based on light frequency!" << std::endl;
    std::cout << "- Blue components (high frequency) = faster rings" << std::endl;
    std::cout << "- Red components (low frequency) = slower rings" << std::endl;
    std::cout << "- Green components = medium speed contribution" << std::endl;
    std::cout << "- Colors are ordered from slowest (red) to fastest (white)" << std::endl;
    std::cout << std::endl;
    std::cout << "Enhanced Wave Interference:" << std::endl;
    std::cout << "- All ring intersections (main + bounces) create atoms" << std::endl;
    std::cout << "- Global atom pool with 35 atom limit (FIFO replacement)" << std::endl;
    std::cout << "- Atoms are independent and persist after rings separate" << std::endl;
    std::cout << "- Same frequency rings cancel out (no interference)" << std::endl;
    std::cout << "- Atom colors = additive mixing of ring frequencies" << std::endl;
    std::cout << "- Higher frequency differences = more energetic atoms" << std::endl;
    std::cout << "- Complex multi-bounce interference patterns!" << std::endl;
    std::cout << std::endl;
    std::cout << "OPTIMIZED BUILD - Performance Improvements:" << std::endl;
    std::cout << "- Spatial grid partitioning (O(n) instead of O(n^2))" << std::endl;
    std::cout << "- Batch rendering (1 draw call vs 100+)" << std::endl;
    std::cout << "- Aggressive bounce shape culling" << std::endl;
    std::cout << "- Squared distance calculations (no sqrt)" << std::endl;
    std::cout << "- Numeric intersection keys (no string allocations)" << std::endl;
    std::cout << std::endl;
    std::cout << "Current frequency: " << ringManager.getCurrentFrequencyInfo() << std::endl;
    std::cout << std::endl;

    // Main game loop
    while (window.isOpen())
    {
        float deltaTime = clock.restart().asSeconds();

        // OPTIMIZED: Clamp delta time to prevent physics explosions on lag spikes
        // Minimum simulated FPS of 10 (max delta = 0.1s)
        deltaTime = std::min(deltaTime, 0.1f);

        // Handle events
        while (auto event = window.pollEvent())
        {
            if (auto closeEvent = event->getIf<sf::Event::Closed>())
            {
                window.close();
            }
            else if (auto mouseClick = event->getIf<sf::Event::MouseButtonPressed>())
            {
                sf::Vector2i clickPos = mouseClick->position;

                if (mouseClick->button == sf::Mouse::Button::Left)
                {
                    // Create ring at click position
                    ringManager.addRing(sf::Vector2f(static_cast<float>(clickPos.x),
                        static_cast<float>(clickPos.y)));

                    // OPTIMIZED: Removed debug spam
                    // std::cout << "Ring created at (" << clickPos.x << ", " << clickPos.y
                    //     << ") - " << ringManager.getCurrentFrequencyInfo() << std::endl;
                }
                else if (mouseClick->button == sf::Mouse::Button::Right)
                {
                    // Cycle to next color
                    ringManager.cycleToNextColor();
                    // OPTIMIZED: Removed debug spam
                    // std::cout << "Frequency changed to: " << ringManager.getCurrentFrequencyInfo() << std::endl;
                }
            }
            else if (auto keyPress = event->getIf<sf::Event::KeyPressed>())
            {
                if (keyPress->code == sf::Keyboard::Key::Space)
                {
                    ringManager.clear();
                    atomManager.clear();
                    std::cout << "All rings and atoms cleared" << std::endl;
                }
                else if (keyPress->code == sf::Keyboard::Key::D)
                {
                    debugShowAtoms = !debugShowAtoms;
                    std::cout << "Debug atom visualization: " << (debugShowAtoms ? "ON" : "OFF") << std::endl;
                }
                else if (keyPress->code == sf::Keyboard::Key::Escape)
                {
                    window.close();
                }
            }
        }

        // Update all rings
        ringManager.update(deltaTime, window.getSize());

        // Update atoms and detect intersections
        std::vector<Ring*> allRings = ringManager.getAllRings();
        atomManager.update(deltaTime, allRings, window.getSize());

        // OPTIMIZED: Removed periodic status spam (was every 5 seconds)
        // Use this for debugging if needed
        /*
        if (infoTimer.getElapsedTime().asSeconds() > 5.0f)
        {
            size_t ringCount = ringManager.getRingCount();
            size_t atomCount = atomManager.getAtomCount();

            if (ringCount > 0 || atomCount > 0)
            {
                std::cout << "Active rings: " << ringCount
                    << ", Active atoms: " << atomCount << "/" << atomManager.getMaxAtoms() << std::endl;
            }
            infoTimer.restart();
        }
        */

        // Render
        window.clear(sf::Color::Black);

        // OPTIMIZED: Use batch rendering (atoms invisible by default, rings always visible)
        batchRenderer.begin();
        ringManager.addToBatch(batchRenderer);
        if (debugShowAtoms) {
            atomManager.addToBatch(batchRenderer);  // Only render atoms in debug mode
        }
        batchRenderer.end(window);

        window.display();
    }

    std::cout << std::endl;
    std::cout << "=== SIMULATION ENDED ===" << std::endl;
    std::cout << "Thank you for exploring enhanced wave interference physics!" << std::endl;

    return 0;
}