use super::*;

#[test]
fn create_game_test() {
    // Arrange
    let position = Position {
        x: 1.0,
        y: 1.0,
        z: 5.0,
    };
    let color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    // Act
    let vertex = Vertex::new(position, color);

    // Assert
    assert_eq!(vertex, Vertex { position, color });
}

#[test]
fn get_position_test() {
    // Arrange
    let position = Position {
        x: 2.0,
        y: 0.0,
        z: 1.0,
    };
    let color = Color {
        r: 0.0,
        g: 0.2,
        b: 1.0,
    };

    let vertex = Vertex::new(position, color);

    // Act
    let result = vertex.get_position();

    // Assert
    assert_eq!(result, &position);
}

#[test]
fn get_color_test() {
    // Arrange
    let position = Position {
        x: 2.0,
        y: 0.0,
        z: 1.0,
    };
    let color = Color {
        r: 0.0,
        g: 0.2,
        b: 1.0,
    };

    let vertex = Vertex::new(position, color);

    // Act
    let result = vertex.get_color();

    // Assert
    assert_eq!(result, &color);
}

#[test]
fn set_position_test() {
    // Arrange
    let position = Position {
        x: 2.0,
        y: 0.0,
        z: 1.0,
    };
    let position_target = Position {
        x: f32::MAX,
        y: 0.0,
        z: 1.0,
    };
    let color = Color {
        r: 0.0,
        g: 0.2,
        b: 1.0,
    };

    let mut vertex = Vertex::new(position, color);

    // Act
    vertex.set_position(position_target);

    // Assert
    assert_eq!(vertex.get_position(), &position_target);
}

#[test]
fn set_color_test() {
    // Arrange
    let position = Position {
        x: 2.0,
        y: 0.0,
        z: 1.0,
    };
    let color = Color {
        r: 0.0,
        g: 0.2,
        b: 1.0,
    };
    let color_target = Color {
        r: f32::MAX,
        g: 0.9,
        b: 0.1,
    };

    let mut vertex = Vertex::new(position, color);

    // Act
    vertex.set_color(color_target);

    // Assert
    assert_eq!(vertex.get_color(), &color_target);
}
