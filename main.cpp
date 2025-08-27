#include <SFML/Graphics.hpp>

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