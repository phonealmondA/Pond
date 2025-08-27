#include <SFML/Graphics.hpp>
#include <iostream>

int main()
{
    // Create a window with 800x600 resolution - SFML 3.0 syntax
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Circle Demo");

    // Create a circle shape
    sf::CircleShape circle(50.f); // radius of 50 pixels
    circle.setFillColor(sf::Color::Red);
    circle.setPosition({ 375.f, 275.f }); // Use sf::Vector2f initialization

    // Main game loop
    while (window.isOpen())
    {
        // Handle events - SFML 3.0 changed event handling
        while (auto event = window.pollEvent())
        {
            // Close window when X button is clicked
            if (event->is<sf::Event::Closed>())
                window.close();

            // Handle mouse button press events
            if (auto mouseClick = event->getIf<sf::Event::MouseButtonPressed>())
            {
                // Get the click position
                sf::Vector2i clickPos = mouseClick->position;

                // Print click information to console
                std::cout << "Mouse clicked at: (" << clickPos.x << ", " << clickPos.y << ")" << std::endl;

                // Check which mouse button was pressed
                if (mouseClick->button == sf::Mouse::Button::Left)
                {
                    std::cout << "Left mouse button pressed!" << std::endl;

                    // Example: Move circle to click position
                    // Offset by circle radius to center it on the click
                    circle.setPosition({ static_cast<float>(clickPos.x - 50),
                                       static_cast<float>(clickPos.y - 50) });
                }
                else if (mouseClick->button == sf::Mouse::Button::Right)
                {
                    std::cout << "Right mouse button pressed!" << std::endl;

                    // Example: Change circle color on right click
                    static int colorIndex = 0;
                    sf::Color colors[] = { sf::Color::Red, sf::Color::Blue, sf::Color::Green, sf::Color::Yellow };
                    colorIndex = (colorIndex + 1) % 4;
                    circle.setFillColor(colors[colorIndex]);
                }
            }

            // Optional: Handle mouse button release events
            if (auto mouseRelease = event->getIf<sf::Event::MouseButtonReleased>())
            {
                sf::Vector2i releasePos = mouseRelease->position;
                std::cout << "Mouse released at: (" << releasePos.x << ", " << releasePos.y << ")" << std::endl;
            }
        }

        // Clear the window with black color (default)
        window.clear(sf::Color::Black);

        // Draw the circle
        window.draw(circle);

        // Display everything on screen
        window.display();
    }

    return 0;
}