#include <SFML/Graphics.hpp>
#include <iostream>
#include "Ring.h"

int main()
{
    // Create a window with 800x600 resolution
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Bouncing Rings");

    // Ring manager to handle all rings
    RingManager ringManager;

    // Clock for timing
    sf::Clock clock;

    // Create an initial center ring to demonstrate bouncing
    ringManager.addRing(sf::Vector2f(400.f, 300.f)); // Center ring

    std::cout << "=== BOUNCING RINGS DEMO ===" << std::endl;
    std::cout << "Controls:" << std::endl;
    std::cout << "- Left click: Create ring at cursor position" << std::endl;
    std::cout << "- Right click: Change ring color" << std::endl;
    std::cout << "- Space: Clear all rings" << std::endl;
    std::cout << "- Escape: Exit" << std::endl;
    std::cout << std::endl;
    std::cout << "Watch the rings bounce off the screen edges!" << std::endl;
    std::cout << "Try clicking near corners for interesting patterns." << std::endl;
    std::cout << "Current color: " << ringManager.getCurrentColorString() << std::endl;
    std::cout << std::endl;

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
                    std::cout << "Bouncing ring created at (" << clickPos.x << ", " << clickPos.y
                        << ") with color: " << ringManager.getCurrentColorString() << std::endl;
                }
                else if (mouseClick->button == sf::Mouse::Button::Right)
                {
                    // Cycle to next color
                    ringManager.cycleToNextColor();
                    std::cout << "Color changed to: " << ringManager.getCurrentColorString() << std::endl;
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

        // Update all rings
        ringManager.update(deltaTime, window.getSize());

        // Render
        window.clear(sf::Color::Black);
        ringManager.draw(window);
        window.display();
    }

    return 0;
}