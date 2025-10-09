#pragma once
#include <SFML/Graphics.hpp>
#include <vector>

// Batch renderer to reduce draw calls significantly
// Instead of drawing each circle individually, we batch them into a single vertex array
class BatchRenderer
{
private:
    sf::VertexArray m_vertices;
    std::vector<sf::Vertex> m_tempVertices;

    // Circle approximation quality
    static const int CIRCLE_SEGMENTS = 24; // Good balance between quality and performance

    // Generate vertices for a circle outline
    void generateCircleOutline(sf::Vector2f center, float radius, sf::Color color, float thickness);

    // Generate vertices for a filled circle
    void generateFilledCircle(sf::Vector2f center, float radius, sf::Color color);

public:
    BatchRenderer();

    // Start a new batch
    void begin();

    // Add a ring (outline circle) to the batch
    void addRing(sf::Vector2f center, float radius, sf::Color color, float thickness = 3.0f);

    // Add an atom (filled circle) to the batch
    void addAtom(sf::Vector2f center, float radius, sf::Color color);

    // Finish the batch and draw everything
    void end(sf::RenderWindow& window);

    // Get current vertex count (for debugging)
    size_t getVertexCount() const { return m_tempVertices.size(); }
};
