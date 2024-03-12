-- Create the 'users' table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Insert dummy data
INSERT INTO users (name, email)
VALUES
    ('Luke Skywalker', 'luke.skywalker@example.com'),
    ('Leia Organa', 'leia.organa@example.com'),
    ('Han Solo', 'han.solo@example.com'),
    ('Darth Vader', 'darth.vader@example.com'),
    ('Obi-Wan Kenobi', 'obi-wan.kenobi@example.com'),
    ('Mara Jade', 'mara.jade@example.com'),
    ('Thrawn', 'thrawn@example.com'),
    ('Qui-Gon Jinn', 'qui-gon.jinn@example.com'),
    ('Ahsoka Tano', 'ahsoka.tano@example.com'),
    ('Kylo Ren', 'kylo.ren@example.com'),
    ('Rey Skywalker', 'rey.skywalker@example.com'),
    ('Revan', 'revan@example.com'),
    ('Bastila Shan', 'bastila.shan@example.com'),
    ('Mara Jade Skywalker', 'mara.jade.skywalker@example.com'),
    ('Jaina Solo', 'jaina.solo@example.com'),
    ('Jacen Solo', 'jacen.solo@example.com'),
    ('Anakin Solo', 'anakin.solo@example.com'),
    ('Tahiri Veila', 'tahiri.veila@example.com'),
    ('Darth Bane', 'darth.bane@example.com'),
    ('Meetra Surik', 'meetra.surik@example.com');
