#include <SFML/Graphics.hpp>
#include <iostream>
#include "Ring.h"

int main()
{
    // Create a window with 800x600 resolution
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Frequency-Based Bouncing Rings with Wave Interference");

    // Ring manager to handle all rings
    RingManager ringManager;

    // Clock for timing
    sf::Clock clock;
    sf::Clock infoTimer; // For periodic ring count display

    // Create an initial center ring to demonstrate bouncing
    ringManager.addRing(sf::Vector2f(400.f, 300.f)); // Center ring

    std::cout << "=== FREQUENCY-BASED BOUNCING RINGS WITH WAVE INTERFERENCE ===" << std::endl;
    std::cout << "Controls:" << std::endl;
    std::cout << "- Left click: Create ring at cursor position" << std::endl;
    std::cout << "- Right click: Change ring color/frequency" << std::endl;
    std::cout << "- Space: Clear all rings" << std::endl;
    std::cout << "- Escape: Exit" << std::endl;
    std::cout << std::endl;
    std::cout << "Wave Physics:" << std::endl;
    std::cout << "- Ring speed is based on light frequency!" << std::endl;
    std::cout << "- Blue components (high frequency) = faster rings" << std::endl;
    std::cout << "- Red components (low frequency) = slower rings" << std::endl;
    std::cout << "- Green components = medium speed contribution" << std::endl;
    std::cout << "- Colors are ordered from slowest (red) to fastest (white)" << std::endl;
    std::cout << std::endl;
    std::cout << "Wave Interference:" << std::endl;
    std::cout << "- Ring reflections create intersection paths when they cross" << std::endl;
    std::cout << "- Same frequency rings cancel out (no interference)" << std::endl;
    std::cout << "- Path colors = additive mixing of ring frequencies" << std::endl;
    std::cout << "- Higher frequency differences = more energetic paths" << std::endl;
    std::cout << "- Interference paths follow moving intersection points" << std::endl;
    std::cout << "- Maximum 6 intersection paths per ring" << std::endl;
    std::cout << std::endl;
    std::cout << "Current frequency: " << ringManager.getCurrentFrequencyInfo() << std::endl;
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

                    std::cout << "Ring created at (" << clickPos.x << ", " << clickPos.y
                        << ") - " << ringManager.getCurrentFrequencyInfo() << std::endl;
                }
                else if (mouseClick->button == sf::Mouse::Button::Right)
                {
                    // Cycle to next color
                    ringManager.cycleToNextColor();
                    std::cout << "Frequency changed to: " << ringManager.getCurrentFrequencyInfo() << std::endl;
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

        // Update all rings and intersection paths
        ringManager.update(deltaTime, window.getSize());

        // Periodic info display (every 5 seconds)
        if (infoTimer.getElapsedTime().asSeconds() > 5.0f)
        {
            size_t ringCount = ringManager.getRingCount();

            if (ringCount > 0)
            {
                std::cout << "Active rings: " << ringCount
                    << " (up to " << (ringCount * 6) << " possible intersection paths)" << std::endl;
            }
            infoTimer.restart();
        }

        // Render
        window.clear(sf::Color::Black);
        ringManager.draw(window);
        window.display();
    }

    std::cout << std::endl;
    std::cout << "=== SIMULATION ENDED ===" << std::endl;
    std::cout << "Thank you for exploring wave interference physics!" << std::endl;

    return 0;
}