#include "BatchRenderer.h"
#include <cmath>

BatchRenderer::BatchRenderer()
    : m_vertices(sf::PrimitiveType::Triangles)
{
    // Reserve space to minimize allocations
    m_tempVertices.reserve(Constants::Rendering::VERTEX_RESERVE_SIZE); // Enough for many circles
}

void BatchRenderer::begin()
{
    m_tempVertices.clear();
}

void BatchRenderer::generateCircleOutline(sf::Vector2f center, float radius, sf::Color color, float thickness)
{
    // Generate circle outline using triangle strip approach
    // This creates a thick ring by drawing triangles between inner and outer circles

    float innerRadius = radius - thickness * 0.5f;
    float outerRadius = radius + thickness * 0.5f;

    if (innerRadius < 0) innerRadius = 0;

    for (int i = 0; i < Constants::System::CIRCLE_SEGMENTS; ++i)
    {
        float angle1 = (static_cast<float>(i) / Constants::System::CIRCLE_SEGMENTS) * 2.0f * Constants::Math::PI;
        float angle2 = (static_cast<float>(i + 1) / Constants::System::CIRCLE_SEGMENTS) * 2.0f * Constants::Math::PI;

        // Outer vertices
        sf::Vector2f outer1(center.x + std::cos(angle1) * outerRadius,
                           center.y + std::sin(angle1) * outerRadius);
        sf::Vector2f outer2(center.x + std::cos(angle2) * outerRadius,
                           center.y + std::sin(angle2) * outerRadius);

        // Inner vertices
        sf::Vector2f inner1(center.x + std::cos(angle1) * innerRadius,
                           center.y + std::sin(angle1) * innerRadius);
        sf::Vector2f inner2(center.x + std::cos(angle2) * innerRadius,
                           center.y + std::sin(angle2) * innerRadius);

        // Create two triangles for this segment
        // Triangle 1
        sf::Vertex v1; v1.position = outer1; v1.color = color;
        sf::Vertex v2; v2.position = inner1; v2.color = color;
        sf::Vertex v3; v3.position = outer2; v3.color = color;
        m_tempVertices.push_back(v1);
        m_tempVertices.push_back(v2);
        m_tempVertices.push_back(v3);

        // Triangle 2
        sf::Vertex v4; v4.position = outer2; v4.color = color;
        sf::Vertex v5; v5.position = inner1; v5.color = color;
        sf::Vertex v6; v6.position = inner2; v6.color = color;
        m_tempVertices.push_back(v4);
        m_tempVertices.push_back(v5);
        m_tempVertices.push_back(v6);
    }
}

void BatchRenderer::generateFilledCircle(sf::Vector2f center, float radius, sf::Color color)
{
    // Generate filled circle using triangle fan
    for (int i = 0; i < Constants::System::CIRCLE_SEGMENTS; ++i)
    {
        float angle1 = (static_cast<float>(i) / Constants::System::CIRCLE_SEGMENTS) * 2.0f * Constants::Math::PI;
        float angle2 = (static_cast<float>(i + 1) / Constants::System::CIRCLE_SEGMENTS) * 2.0f * Constants::Math::PI;

        sf::Vector2f p1(center.x + std::cos(angle1) * radius,
                       center.y + std::sin(angle1) * radius);
        sf::Vector2f p2(center.x + std::cos(angle2) * radius,
                       center.y + std::sin(angle2) * radius);

        // Create triangle from center to edge
        sf::Vertex v1; v1.position = center; v1.color = color;
        sf::Vertex v2; v2.position = p1; v2.color = color;
        sf::Vertex v3; v3.position = p2; v3.color = color;
        m_tempVertices.push_back(v1);
        m_tempVertices.push_back(v2);
        m_tempVertices.push_back(v3);
    }
}

void BatchRenderer::addRing(sf::Vector2f center, float radius, sf::Color color, float thickness)
{
    generateCircleOutline(center, radius, color, thickness);
}

void BatchRenderer::addAtom(sf::Vector2f center, float radius, sf::Color color)
{
    generateFilledCircle(center, radius, color);
}

void BatchRenderer::end(sf::RenderWindow& window)
{
    // Copy temp vertices to vertex array
    m_vertices.resize(m_tempVertices.size());
    for (size_t i = 0; i < m_tempVertices.size(); ++i)
    {
        m_vertices[i] = m_tempVertices[i];
    }

    // Draw everything in a single call!
    window.draw(m_vertices);
}
