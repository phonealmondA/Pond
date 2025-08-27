#include "events.h"
#include <iostream>

void handleWindowEvents(const sf::Event::Closed* closeEvent, sf::RenderWindow& window)
{
    if (closeEvent)
    {
        window.close();
    }
}

void handleMouseButtonPress(const sf::Event::MouseButtonPressed* mouseClick, sf::CircleShape& shape, float currentRadius)
{
    if (!mouseClick) return;

    // Get the click position
    sf::Vector2i clickPos = mouseClick->position;

    // Print click information to console
    std::cout << "Mouse clicked at: (" << clickPos.x << ", " << clickPos.y << ")" << std::endl;

    // Check which mouse button was pressed
    if (mouseClick->button == sf::Mouse::Button::Left)
    {
        std::cout << "Left mouse button pressed!" << std::endl;

        // Reset ring to small size and center it on click position - SFML 3.0 compatible
        float newRadius = 10.f;
        shape.setRadius(newRadius);
        shape.setPosition(sf::Vector2f(static_cast<float>(clickPos.x - newRadius),
            static_cast<float>(clickPos.y - newRadius)));

        std::cout << "Shape reset and centered at click position" << std::endl;
    }
    else if (mouseClick->button == sf::Mouse::Button::Right)
    {
        std::cout << "Right mouse button pressed!" << std::endl;

        // Change ring color on right click
        static int colorIndex = 0;
        sf::Color colors[] = { sf::Color::Red, sf::Color::Blue, sf::Color::Green,
                             sf::Color::Yellow, sf::Color::Magenta, sf::Color::Cyan };
        colorIndex = (colorIndex + 1) % 6;
        shape.setOutlineColor(colors[colorIndex]);
    }
}

void handleMouseButtonRelease(const sf::Event::MouseButtonReleased* mouseRelease)
{
    if (!mouseRelease) return;

    sf::Vector2i releasePos = mouseRelease->position;
    std::cout << "Mouse released at: (" << releasePos.x << ", " << releasePos.y << ")" << std::endl;
}