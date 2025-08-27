#pragma once
#include <SFML/Graphics.hpp>

// Function declarations for event handling
void handleWindowEvents(const sf::Event::Closed* closeEvent, sf::RenderWindow& window);
void handleMouseButtonPress(const sf::Event::MouseButtonPressed* mouseClick, sf::CircleShape& shape, float currentRadius);
void handleMouseButtonRelease(const sf::Event::MouseButtonReleased* mouseRelease);