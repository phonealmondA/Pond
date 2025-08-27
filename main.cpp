#include <SFML/Graphics.hpp>
#include <iostream>
#include "Ring.h"

int main()
{
    // Create a window with 800x600 resolution
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Multiple Growing Rings");

    // Ring manager to handle all rings
    RingManager ringManager;

    // Clock for timing
    sf::Clock clock;
    sf::Clock autoSpawnClock; // For automatic ring spawning

    // Create some initial rings
    ringManager.addRing(sf::Vector2f(400.f, 300.f)); // Center ring

    std::cout << "Controls:" << std::endl;
    std::cout << "- Left click: Create ring at cursor position" << std::endl;
    std::cout << "- Right click: Create random ring" << std::endl;
    std::cout << "- Space: Clear all rings" << std::endl;
    std::cout << "- Escape: Exit" << std::endl;

    // Main game loop
    while (window.isOpen())
    {
        float deltaTime = clock.restart().asSeconds();

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
                    std::cout << "Ring created at (" << clickPos.x << ", " << clickPos.y << ")" << std::endl;
                }
                else if (mouseClick->button == sf::Mouse::Button::Right)
                {
                    // Create random ring
                    ringManager.addRandomRing(window.getSize());
                    std::cout << "Random ring created" << std::endl;
                }
            }
            else if (auto keyPress = event->getIf<sf::Event::KeyPressed>())
            {
                if (keyPress->code == sf::Keyboard::Key::Space)
                {
                    ringManager.clear();
                    std::cout << "All rings cleared" << std::endl;
                }
                else if (keyPress->code == sf::Keyboard::Key::Escape)
                {
                    window.close();
                }
            }
        }

        // Auto-spawn rings occasionally (every 3 seconds if less than 5 rings)
        if (autoSpawnClock.getElapsedTime().asSeconds() > 3.0f && ringManager.getRingCount() < 5)
        {
            ringManager.addRandomRing(window.getSize());
            autoSpawnClock.restart();
        }

        // Update all rings
        ringManager.update(deltaTime, window.getSize());

        // Render
        window.clear(sf::Color::Black);
        ringManager.draw(window);
        window.display();
    }

    return 0;
}