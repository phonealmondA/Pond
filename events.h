#pragma once
#include <SFML/Graphics.hpp>

// Simple event handler functions - not needed for the Ring system but kept for compatibility
void handleWindowEvents(const sf::Event::Closed* closeEvent, sf::RenderWindow& window);
void handleMouseButtonPress(const sf::Event::MouseButtonPressed* mouseClick, sf::CircleShape& shape, float currentRadius);
void handleMouseButtonRelease(const sf::Event::MouseButtonReleased* mouseRelease);