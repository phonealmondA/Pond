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

        // Move ring to click position - center it on the click using current radius
        shape.setPosition({ static_cast<float>(clickPos.x - currentRadius),
                           static_cast<float>(clickPos.y - currentRadius) });
    }
    else if (mouseClick->button == sf::Mouse::Button::Right)
    {
        std::cout << "Right mouse button pressed!" << std::endl;

        // Change ring color on right click
        static int colorIndex = 0;
        sf::Color colors[] = { sf::Color::Red, sf::Color::Blue, sf::Color::Green, sf::Color::Yellow };
        colorIndex = (colorIndex + 1) % 4;
        shape.setOutlineColor(colors[colorIndex]); // Change outline color for ring
    }
}

void handleMouseButtonRelease(const sf::Event::MouseButtonReleased* mouseRelease)
{
    if (!mouseRelease) return;

    sf::Vector2i releasePos = mouseRelease->position;
    std::cout << "Mouse released at: (" << releasePos.x << ", " << releasePos.y << ")" << std::endl;
}