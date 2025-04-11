// if keyboard.just_pressed(KeyCode::KeyT) {
//     for (_, config, _) in config_store.iter_mut() {
//         config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
//     }
// }
// if keyboard.just_pressed(KeyCode::KeyP) {
//     for (_, config, _) in config_store.iter_mut() {
//         // Toggle line_perspective
//         config.line_perspective ^= true;
//         // Increase the line width when line_perspective is on
//         config.line_width *= if config.line_perspective { 5. } else { 1. / 5. };
//     }
// }

// if keyboard.pressed(KeyCode::ArrowRight) {
//     config.line_width += 5. * time.delta_secs();
//     config.line_width = config.line_width.clamp(0., 50.);
// }
// if keyboard.pressed(KeyCode::ArrowLeft) {
//     config.line_width -= 5. * time.delta_secs();
//     config.line_width = config.line_width.clamp(0., 50.);
// }

// if keyboard.just_pressed(KeyCode::KeyU) {
//     config.line_style = match config.line_style {
//         GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
//         _ => GizmoLineStyle::Solid,
//     };
// }
// if keyboard.just_pressed(KeyCode::KeyJ) {
//     config.line_joints = match config.line_joints {
//         GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
//         GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
//         GizmoLineJoint::Round(_) => GizmoLineJoint::None,
//         GizmoLineJoint::None => GizmoLineJoint::Bevel,
//     };
// }

// let (my_config, _) = config_store.config_mut::<MagmagridGizmos>();
// if keyboard.pressed(KeyCode::ArrowUp) {
//     my_config.line_width += 5. * time.delta_secs();
//     my_config.line_width = my_config.line_width.clamp(0., 50.);
// }
// if keyboard.pressed(KeyCode::ArrowDown) {
//     my_config.line_width -= 5. * time.delta_secs();
//     my_config.line_width = my_config.line_width.clamp(0., 50.);
// }
// if keyboard.just_pressed(KeyCode::Digit2) {
//     my_config.enabled ^= true;
// }
// if keyboard.just_pressed(KeyCode::KeyI) {
//     my_config.line_style = match my_config.line_style {
//         GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
//         _ => GizmoLineStyle::Solid,
//     };
// }
// if keyboard.just_pressed(KeyCode::KeyK) {
//     my_config.line_joints = match my_config.line_joints {
//         GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
//         GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
//         GizmoLineJoint::Round(_) => GizmoLineJoint::None,
//         GizmoLineJoint::None => GizmoLineJoint::Bevel,
//     };
// }

// if keyboard.just_pressed(KeyCode::KeyB) {
//     // AABB gizmos are normally only drawn on entities with a ShowAabbGizmo component
//     // We can change this behavior in the configuration of AabbGizmoGroup
//     config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
// }
