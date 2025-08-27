#include <SFML/Graphics.hpp>
#include <iostream>
#include <cmath>
#include "events.h"

int main()
{
    // Create a window with 800x600 resolution - SFML 3.0 syntax
    sf::RenderWindow window(sf::VideoMode({ 800, 600 }), "Pond - Ring Demo");

    // Create a ring shape (circle with outline, no fill)
    sf::CircleShape circle(50.f); // radius of 50 pixels
    circle.setFillColor(sf::Color::Transparent); // No fill to create ring effect
    circle.setOutlineThickness(10.f); // Ring thickness
    circle.setOutlineColor(sf::Color::Red); // Ring color
    circle.setPosition({ 375.f, 275.f }); // Use sf::Vector2f initialization

    // Clock for timing animations
    sf::Clock clock;

    // Main game loop
    while (window.isOpen())
    {
        // Get elapsed time for animations
        float time = clock.getElapsedTime().asSeconds();

        // Animate the ring size - pulsing effect
        float baseRadius = 50.f;
        float pulseAmount = 20.f;
        float newRadius = baseRadius + pulseAmount * std::sin(time * 2.0f); // 2.0f controls speed
        circle.setRadius(newRadius);

        // Handle events - SFML 3.0 changed event handling
        while (auto event = window.pollEvent())
        {
            // Handle different event types using separate functions
            handleWindowEvents(event->getIf<sf::Event::Closed>(), window);
            handleMouseButtonPress(event->getIf<sf::Event::MouseButtonPressed>(), circle, newRadius);
            handleMouseButtonRelease(event->getIf<sf::Event::MouseButtonReleased>());
        }

        // Clear the window with black color (default)
        window.clear(sf::Color::Black);

        // Draw the ring
        window.draw(circle);

        // Display everything on screen
        window.display();
    }

    return 0;
}