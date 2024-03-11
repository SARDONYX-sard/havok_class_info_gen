mod bgs_gamebryo_sequence_generator;
use bgs_gamebryo_sequence_generator::*;

mod bs_bone_switch_generator_bone_data;
use bs_bone_switch_generator_bone_data::*;

mod bs_bone_switch_generator;
use bs_bone_switch_generator::*;

mod bs_compute_add_bone_anim_modifier;
use bs_compute_add_bone_anim_modifier::*;

mod bs_cyclic_blend_transition_generator;
use bs_cyclic_blend_transition_generator::*;

mod bs_decompose_vector_modifier;
use bs_decompose_vector_modifier::*;

mod bs_direct_at_modifier;
use bs_direct_at_modifier::*;

mod bs_dist_trigger_modifier;
use bs_dist_trigger_modifier::*;

mod bs_event_every_n_events_modifier;
use bs_event_every_n_events_modifier::*;

mod bs_event_on_deactivate_modifier;
use bs_event_on_deactivate_modifier::*;

mod bs_event_on_false_to_true_modifier;
use bs_event_on_false_to_true_modifier::*;

mod bs_get_time_step_modifier;
use bs_get_time_step_modifier::*;

mod bs_interp_value_modifier;
use bs_interp_value_modifier::*;

mod bs_is_active_modifier;
use bs_is_active_modifier::*;

mod bsi_state_manager_modifier_b_si_state_data;
use bsi_state_manager_modifier_b_si_state_data::*;

mod bsi_state_manager_modifier_bsi_state_manager_state_listener;
use bsi_state_manager_modifier_bsi_state_manager_state_listener::*;

mod bsi_state_manager_modifier;
use bsi_state_manager_modifier::*;

mod b_si_state_tagging_generator;
use b_si_state_tagging_generator::*;

mod bs_limb_ik_modifier;
use bs_limb_ik_modifier::*;

mod bs_look_at_modifier_bone_data;
use bs_look_at_modifier_bone_data::*;

mod bs_look_at_modifier;
use bs_look_at_modifier::*;

mod bs_modify_once_modifier;
use bs_modify_once_modifier::*;

mod bs_offset_animation_generator;
use bs_offset_animation_generator::*;

mod bs_pass_by_target_trigger_modifier;
use bs_pass_by_target_trigger_modifier::*;

mod bs_ragdoll_contact_listener_modifier;
use bs_ragdoll_contact_listener_modifier::*;

mod bs_speed_sampler_modifier;
use bs_speed_sampler_modifier::*;

mod bs_synchronized_clip_generator;
use bs_synchronized_clip_generator::*;

mod bs_timer_modifier;
use bs_timer_modifier::*;

mod bs_tweener_modifier;
use bs_tweener_modifier::*;

mod hk_aabb_half;
use hk_aabb_half::*;

mod hk_aabb_uint_32;
use hk_aabb_uint_32::*;

mod hk_aabb;
use hk_aabb::*;

mod hka_animated_reference_frame;
use hka_animated_reference_frame::*;

mod hka_animation_binding;
use hka_animation_binding::*;

mod hka_animation_container;
use hka_animation_container::*;

mod hka_animation_preview_color_container;
use hka_animation_preview_color_container::*;

mod hka_animation;
use hka_animation::*;

mod hka_annotation_track_annotation;
use hka_annotation_track_annotation::*;

mod hka_annotation_track;
use hka_annotation_track::*;

mod hka_bone_attachment;
use hka_bone_attachment::*;

mod hka_bone;
use hka_bone::*;

mod hka_default_animated_reference_frame;
use hka_default_animated_reference_frame::*;

mod hka_delta_compressed_animation_quantization_format;
use hka_delta_compressed_animation_quantization_format::*;

mod hka_delta_compressed_animation;
use hka_delta_compressed_animation::*;

mod hka_footstep_analysis_info_container;
use hka_footstep_analysis_info_container::*;

mod hka_footstep_analysis_info;
use hka_footstep_analysis_info::*;

mod hka_interleaved_uncompressed_animation;
use hka_interleaved_uncompressed_animation::*;

mod hka_key_frame_hierarchy_utility_control_data;
use hka_key_frame_hierarchy_utility_control_data::*;

mod hka_key_frame_hierarchy_utility;
use hka_key_frame_hierarchy_utility::*;

mod hk_align_scene_to_node_options;
use hk_align_scene_to_node_options::*;

mod hka_mesh_binding_mapping;
use hka_mesh_binding_mapping::*;

mod hka_mesh_binding;
use hka_mesh_binding::*;

mod hka_quantized_animation_track_compression_params;
use hka_quantized_animation_track_compression_params::*;

mod hka_quantized_animation;
use hka_quantized_animation::*;

mod hka_ragdoll_instance;
use hka_ragdoll_instance::*;

mod hk_array_type_attribute;
use hk_array_type_attribute::*;

mod hka_skeleton_local_frame_on_bone;
use hka_skeleton_local_frame_on_bone::*;

mod hka_skeleton_mapper_data_chain_mapping;
use hka_skeleton_mapper_data_chain_mapping::*;

mod hka_skeleton_mapper_data_simple_mapping;
use hka_skeleton_mapper_data_simple_mapping::*;

mod hka_skeleton_mapper_data;
use hka_skeleton_mapper_data::*;

mod hka_skeleton_mapper;
use hka_skeleton_mapper::*;

mod hka_skeleton;
use hka_skeleton::*;

mod hka_spline_compressed_animation_animation_compression_params;
use hka_spline_compressed_animation_animation_compression_params::*;

mod hka_spline_compressed_animation_track_compression_params;
use hka_spline_compressed_animation_track_compression_params::*;

mod hka_spline_compressed_animation;
use hka_spline_compressed_animation::*;

mod hka_wavelet_compressed_animation_compression_params;
use hka_wavelet_compressed_animation_compression_params::*;

mod hka_wavelet_compressed_animation_quantization_format;
use hka_wavelet_compressed_animation_quantization_format::*;

mod hka_wavelet_compressed_animation;
use hka_wavelet_compressed_animation::*;

mod hk_base_object;
use hk_base_object::*;

mod hkb_attachment_modifier;
use hkb_attachment_modifier::*;

mod hkb_attachment_setup;
use hkb_attachment_setup::*;

mod hkb_attribute_modifier_assignment;
use hkb_attribute_modifier_assignment::*;

mod hkb_attribute_modifier;
use hkb_attribute_modifier::*;

mod hkb_auxiliary_node_info;
use hkb_auxiliary_node_info::*;

mod hkb_behavior_events_info;
use hkb_behavior_events_info::*;

mod hkb_behavior_graph_data;
use hkb_behavior_graph_data::*;

mod hkb_behavior_graph_internal_state_info;
use hkb_behavior_graph_internal_state_info::*;

mod hkb_behavior_graph_internal_state;
use hkb_behavior_graph_internal_state::*;

mod hkb_behavior_graph_string_data;
use hkb_behavior_graph_string_data::*;

mod hkb_behavior_graph;
use hkb_behavior_graph::*;

mod hkb_behavior_info_id_to_name_pair;
use hkb_behavior_info_id_to_name_pair::*;

mod hkb_behavior_info;
use hkb_behavior_info::*;

mod hkb_behavior_reference_generator;
use hkb_behavior_reference_generator::*;

mod hkb_bindable;
use hkb_bindable::*;

mod hkb_blend_curve_utils;
use hkb_blend_curve_utils::*;

mod hkb_blender_generator_child_internal_state;
use hkb_blender_generator_child_internal_state::*;

mod hkb_blender_generator_child;
use hkb_blender_generator_child::*;

mod hkb_blender_generator_internal_state;
use hkb_blender_generator_internal_state::*;

mod hkb_blender_generator;
use hkb_blender_generator::*;

mod hkb_blending_transition_effect_internal_state;
use hkb_blending_transition_effect_internal_state::*;

mod hkb_blending_transition_effect;
use hkb_blending_transition_effect::*;

mod hkb_bone_index_array;
use hkb_bone_index_array::*;

mod hkb_bone_weight_array;
use hkb_bone_weight_array::*;

mod hkb_bool_variable_sequenced_data_sample;
use hkb_bool_variable_sequenced_data_sample::*;

mod hkb_bool_variable_sequenced_data;
use hkb_bool_variable_sequenced_data::*;

mod hkb_camera_shake_event_payload;
use hkb_camera_shake_event_payload::*;

mod hkb_character_added_info;
use hkb_character_added_info::*;

mod hkb_character_control_command;
use hkb_character_control_command::*;

mod hkb_character_controller_control_data;
use hkb_character_controller_control_data::*;

mod hkb_character_controller_modifier_internal_state;
use hkb_character_controller_modifier_internal_state::*;

mod hkb_character_controller_modifier;
use hkb_character_controller_modifier::*;

mod hkb_character_data_character_controller_info;
use hkb_character_data_character_controller_info::*;

mod hkb_character_data;
use hkb_character_data::*;

mod hkb_character_info;
use hkb_character_info::*;

mod hkb_character_setup;
use hkb_character_setup::*;

mod hkb_character_skin_info;
use hkb_character_skin_info::*;

mod hkb_character_stepped_info;
use hkb_character_stepped_info::*;

mod hkb_character_string_data;
use hkb_character_string_data::*;

mod hkb_character;
use hkb_character::*;

mod hkb_client_character_state;
use hkb_client_character_state::*;

mod hkb_clip_generator_echo;
use hkb_clip_generator_echo::*;

mod hkb_clip_generator_internal_state;
use hkb_clip_generator_internal_state::*;

mod hkb_clip_generator;
use hkb_clip_generator::*;

mod hkb_clip_trigger_array;
use hkb_clip_trigger_array::*;

mod hkb_clip_trigger;
use hkb_clip_trigger::*;

mod hkb_combine_transforms_modifier_internal_state;
use hkb_combine_transforms_modifier_internal_state::*;

mod hkb_combine_transforms_modifier;
use hkb_combine_transforms_modifier::*;

mod hkb_compiled_expression_set_token;
use hkb_compiled_expression_set_token::*;

mod hkb_compiled_expression_set;
use hkb_compiled_expression_set::*;

mod hkb_compute_direction_modifier_internal_state;
use hkb_compute_direction_modifier_internal_state::*;

mod hkb_compute_direction_modifier;
use hkb_compute_direction_modifier::*;

mod hkb_compute_rotation_from_axis_angle_modifier_internal_state;
use hkb_compute_rotation_from_axis_angle_modifier_internal_state::*;

mod hkb_compute_rotation_from_axis_angle_modifier;
use hkb_compute_rotation_from_axis_angle_modifier::*;

mod hkb_compute_rotation_to_target_modifier_internal_state;
use hkb_compute_rotation_to_target_modifier_internal_state::*;

mod hkb_compute_rotation_to_target_modifier;
use hkb_compute_rotation_to_target_modifier::*;

mod hkb_condition;
use hkb_condition::*;

mod hkb_context;
use hkb_context::*;

mod hkb_damping_modifier_internal_state;
use hkb_damping_modifier_internal_state::*;

mod hkb_damping_modifier;
use hkb_damping_modifier::*;

mod hkb_default_message_log;
use hkb_default_message_log::*;

mod hkb_delayed_modifier_internal_state;
use hkb_delayed_modifier_internal_state::*;

mod hkb_delayed_modifier;
use hkb_delayed_modifier::*;

mod hkb_detect_close_to_ground_modifier_internal_state;
use hkb_detect_close_to_ground_modifier_internal_state::*;

mod hkb_detect_close_to_ground_modifier;
use hkb_detect_close_to_ground_modifier::*;

mod hkb_evaluate_expression_modifier_internal_expression_data;
use hkb_evaluate_expression_modifier_internal_expression_data::*;

mod hkb_evaluate_expression_modifier_internal_state;
use hkb_evaluate_expression_modifier_internal_state::*;

mod hkb_evaluate_expression_modifier;
use hkb_evaluate_expression_modifier::*;

mod hkb_evaluate_handle_modifier;
use hkb_evaluate_handle_modifier::*;

mod hkb_event_base;
use hkb_event_base::*;

mod hkb_event_driven_modifier_internal_state;
use hkb_event_driven_modifier_internal_state::*;

mod hkb_event_driven_modifier;
use hkb_event_driven_modifier::*;

mod hkb_event_info;
use hkb_event_info::*;

mod hkb_event_payload_list;
use hkb_event_payload_list::*;

mod hkb_event_payload;
use hkb_event_payload::*;

mod hkb_event_property;
use hkb_event_property::*;

mod hkb_event_raised_info;
use hkb_event_raised_info::*;

mod hkb_event_range_data_array;
use hkb_event_range_data_array::*;

mod hkb_event_range_data;
use hkb_event_range_data::*;

mod hkb_event_sequenced_data_sequenced_event;
use hkb_event_sequenced_data_sequenced_event::*;

mod hkb_event_sequenced_data;
use hkb_event_sequenced_data::*;

mod hkb_events_from_range_modifier_internal_state;
use hkb_events_from_range_modifier_internal_state::*;

mod hkb_events_from_range_modifier;
use hkb_events_from_range_modifier::*;

mod hkb_event;
use hkb_event::*;

mod hkb_expression_condition;
use hkb_expression_condition::*;

mod hkb_expression_data_array;
use hkb_expression_data_array::*;

mod hkb_expression_data;
use hkb_expression_data::*;

mod hkb_extract_ragdoll_pose_modifier;
use hkb_extract_ragdoll_pose_modifier::*;

mod hkb_foot_ik_control_data;
use hkb_foot_ik_control_data::*;

mod hkb_foot_ik_controls_modifier_leg;
use hkb_foot_ik_controls_modifier_leg::*;

mod hkb_foot_ik_controls_modifier;
use hkb_foot_ik_controls_modifier::*;

mod hkb_foot_ik_driver_info_leg;
use hkb_foot_ik_driver_info_leg::*;

mod hkb_foot_ik_driver_info;
use hkb_foot_ik_driver_info::*;

mod hkb_foot_ik_gains;
use hkb_foot_ik_gains::*;

mod hkb_foot_ik_modifier_internal_leg_data;
use hkb_foot_ik_modifier_internal_leg_data::*;

mod hkb_foot_ik_modifier_leg;
use hkb_foot_ik_modifier_leg::*;

mod hkb_foot_ik_modifier;
use hkb_foot_ik_modifier::*;

mod hkb_generator_output_listener;
use hkb_generator_output_listener::*;

mod hkb_generator_sync_info_sync_point;
use hkb_generator_sync_info_sync_point::*;

mod hkb_generator_sync_info;
use hkb_generator_sync_info::*;

mod hkb_generator_transition_effect_internal_state;
use hkb_generator_transition_effect_internal_state::*;

mod hkb_generator_transition_effect;
use hkb_generator_transition_effect::*;

mod hkb_generator;
use hkb_generator::*;

mod hkb_get_handle_on_bone_modifier;
use hkb_get_handle_on_bone_modifier::*;

mod hkb_get_up_modifier_internal_state;
use hkb_get_up_modifier_internal_state::*;

mod hkb_get_up_modifier;
use hkb_get_up_modifier::*;

mod hkb_get_world_from_model_modifier_internal_state;
use hkb_get_world_from_model_modifier_internal_state::*;

mod hkb_get_world_from_model_modifier;
use hkb_get_world_from_model_modifier::*;

mod hkb_hand_ik_control_data;
use hkb_hand_ik_control_data::*;

mod hkb_hand_ik_controls_modifier_hand;
use hkb_hand_ik_controls_modifier_hand::*;

mod hkb_hand_ik_controls_modifier;
use hkb_hand_ik_controls_modifier::*;

mod hkb_hand_ik_driver_info_hand;
use hkb_hand_ik_driver_info_hand::*;

mod hkb_hand_ik_driver_info;
use hkb_hand_ik_driver_info::*;

mod hkb_hand_ik_modifier_hand;
use hkb_hand_ik_modifier_hand::*;

mod hkb_hand_ik_modifier;
use hkb_hand_ik_modifier::*;

mod hkb_handle;
use hkb_handle::*;

mod hkb_int_event_payload;
use hkb_int_event_payload::*;

mod hkb_int_variable_sequenced_data_sample;
use hkb_int_variable_sequenced_data_sample::*;

mod hkb_int_variable_sequenced_data;
use hkb_int_variable_sequenced_data::*;

mod hk_bit_field;
use hk_bit_field::*;

mod hkb_keyframe_bones_modifier_keyframe_info;
use hkb_keyframe_bones_modifier_keyframe_info::*;

mod hkb_keyframe_bones_modifier;
use hkb_keyframe_bones_modifier::*;

mod hkb_linked_symbol_info;
use hkb_linked_symbol_info::*;

mod hkb_look_at_modifier_internal_state;
use hkb_look_at_modifier_internal_state::*;

mod hkb_look_at_modifier;
use hkb_look_at_modifier::*;

mod hkb_manual_selector_generator_internal_state;
use hkb_manual_selector_generator_internal_state::*;

mod hkb_manual_selector_generator;
use hkb_manual_selector_generator::*;

mod hkb_message_log;
use hkb_message_log::*;

mod hkb_mirrored_skeleton_info;
use hkb_mirrored_skeleton_info::*;

mod hkb_mirror_modifier;
use hkb_mirror_modifier::*;

mod hkb_modifier_generator;
use hkb_modifier_generator::*;

mod hkb_modifier_list;
use hkb_modifier_list::*;

mod hkb_modifier_wrapper;
use hkb_modifier_wrapper::*;

mod hkb_modifier;
use hkb_modifier::*;

mod hkb_move_character_modifier_internal_state;
use hkb_move_character_modifier_internal_state::*;

mod hkb_move_character_modifier;
use hkb_move_character_modifier::*;

mod hkb_named_event_payload;
use hkb_named_event_payload::*;

mod hkb_named_int_event_payload;
use hkb_named_int_event_payload::*;

mod hkb_named_real_event_payload;
use hkb_named_real_event_payload::*;

mod hkb_named_string_event_payload;
use hkb_named_string_event_payload::*;

mod hkb_node_internal_state_info;
use hkb_node_internal_state_info::*;

mod hkb_node;
use hkb_node::*;

mod hkb_particle_system_event_payload;
use hkb_particle_system_event_payload::*;

mod hkb_pose_matching_generator_internal_state;
use hkb_pose_matching_generator_internal_state::*;

mod hkb_pose_matching_generator;
use hkb_pose_matching_generator::*;

mod hkb_powered_ragdoll_control_data;
use hkb_powered_ragdoll_control_data::*;

mod hkb_powered_ragdoll_controls_modifier;
use hkb_powered_ragdoll_controls_modifier::*;

mod hkb_project_data;
use hkb_project_data::*;

mod hkb_project_string_data;
use hkb_project_string_data::*;

mod hkb_proxy_modifier_proxy_info;
use hkb_proxy_modifier_proxy_info::*;

mod hkb_proxy_modifier;
use hkb_proxy_modifier::*;

mod hkb_raise_event_command;
use hkb_raise_event_command::*;

mod hkb_real_event_payload;
use hkb_real_event_payload::*;

mod hkb_real_variable_sequenced_data_sample;
use hkb_real_variable_sequenced_data_sample::*;

mod hkb_real_variable_sequenced_data;
use hkb_real_variable_sequenced_data::*;

mod hkb_reference_pose_generator;
use hkb_reference_pose_generator::*;

mod hkb_registered_generator;
use hkb_registered_generator::*;

mod hkb_rigid_body_ragdoll_control_data;
use hkb_rigid_body_ragdoll_control_data::*;

mod hkb_rigid_body_ragdoll_controls_modifier;
use hkb_rigid_body_ragdoll_controls_modifier::*;

mod hkb_role_attribute;
use hkb_role_attribute::*;

mod hkb_rotate_character_modifier_internal_state;
use hkb_rotate_character_modifier_internal_state::*;

mod hkb_rotate_character_modifier;
use hkb_rotate_character_modifier::*;

mod hkb_sense_handle_modifier_range;
use hkb_sense_handle_modifier_range::*;

mod hkb_sense_handle_modifier;
use hkb_sense_handle_modifier::*;

mod hkb_sequenced_data;
use hkb_sequenced_data::*;

mod hkb_sequence_internal_state;
use hkb_sequence_internal_state::*;

mod hkb_sequence_string_data;
use hkb_sequence_string_data::*;

mod hkb_sequence;
use hkb_sequence::*;

mod hkb_set_behavior_command;
use hkb_set_behavior_command::*;

mod hkb_set_local_time_of_clip_generator_command;
use hkb_set_local_time_of_clip_generator_command::*;

mod hkb_set_node_property_command;
use hkb_set_node_property_command::*;

mod hkb_set_word_variable_command;
use hkb_set_word_variable_command::*;

mod hkb_set_world_from_model_modifier;
use hkb_set_world_from_model_modifier::*;

mod hkb_simulation_control_command;
use hkb_simulation_control_command::*;

mod hkb_simulation_state_info;
use hkb_simulation_state_info::*;

mod hkb_state_chooser;
use hkb_state_chooser::*;

mod hkb_state_listener;
use hkb_state_listener::*;

mod hkb_state_machine_active_transition_info;
use hkb_state_machine_active_transition_info::*;

mod hkb_state_machine_delayed_transition_info;
use hkb_state_machine_delayed_transition_info::*;

mod hkb_state_machine_event_property_array;
use hkb_state_machine_event_property_array::*;

mod hkb_state_machine_internal_state;
use hkb_state_machine_internal_state::*;

mod hkb_state_machine_nested_state_machine_data;
use hkb_state_machine_nested_state_machine_data::*;

mod hkb_state_machine_prospective_transition_info;
use hkb_state_machine_prospective_transition_info::*;

mod hkb_state_machine_state_info;
use hkb_state_machine_state_info::*;

mod hkb_state_machine_time_interval;
use hkb_state_machine_time_interval::*;

mod hkb_state_machine_transition_info_array;
use hkb_state_machine_transition_info_array::*;

mod hkb_state_machine_transition_info_reference;
use hkb_state_machine_transition_info_reference::*;

mod hkb_state_machine_transition_info;
use hkb_state_machine_transition_info::*;

mod hkb_state_machine;
use hkb_state_machine::*;

mod hkb_string_condition;
use hkb_string_condition::*;

mod hkb_string_event_payload;
use hkb_string_event_payload::*;

mod hkb_test_state_chooser;
use hkb_test_state_chooser::*;

mod hkb_timer_modifier_internal_state;
use hkb_timer_modifier_internal_state::*;

mod hkb_timer_modifier;
use hkb_timer_modifier::*;

mod hkb_transform_vector_modifier_internal_state;
use hkb_transform_vector_modifier_internal_state::*;

mod hkb_transform_vector_modifier;
use hkb_transform_vector_modifier::*;

mod hkb_transition_effect;
use hkb_transition_effect::*;

mod hkb_twist_modifier;
use hkb_twist_modifier::*;

mod hkb_variable_binding_set_binding;
use hkb_variable_binding_set_binding::*;

mod hkb_variable_binding_set;
use hkb_variable_binding_set::*;

mod hkb_variable_info;
use hkb_variable_info::*;

mod hkb_variable_value_set;
use hkb_variable_value_set::*;

mod hkb_variable_value;
use hkb_variable_value::*;

mod hkb_world_enums;
use hkb_world_enums::*;

mod hkb_world_from_model_mode_data;
use hkb_world_from_model_mode_data::*;

mod hk_class_enum_item;
use hk_class_enum_item::*;

mod hk_class_enum;
use hk_class_enum::*;

mod hk_class_member;
use hk_class_member::*;

mod hk_class;
use hk_class::*;

mod hk_color;
use hk_color::*;

mod hk_contact_point_material;
use hk_contact_point_material::*;

mod hk_contact_point;
use hk_contact_point::*;

mod hk_custom_attributes_attribute;
use hk_custom_attributes_attribute::*;

mod hk_custom_attributes;
use hk_custom_attributes::*;

mod hk_data_object_type_attribute;
use hk_data_object_type_attribute::*;

mod hk_description_attribute;
use hk_description_attribute::*;

mod hk_documentation_attribute;
use hk_documentation_attribute::*;

mod hk_geometry_triangle;
use hk_geometry_triangle::*;

mod hk_geometry;
use hk_geometry::*;

mod hk_gizmo_attribute;
use hk_gizmo_attribute::*;

mod hk_half_8;
use hk_half_8::*;

mod hk_indexed_transform_set;
use hk_indexed_transform_set::*;

mod hk_link_attribute;
use hk_link_attribute::*;

mod hk_local_frame_group;
use hk_local_frame_group::*;

mod hk_local_frame;
use hk_local_frame::*;

mod hk_memory_mesh_body;
use hk_memory_mesh_body::*;

mod hk_memory_mesh_material;
use hk_memory_mesh_material::*;

mod hk_memory_mesh_shape;
use hk_memory_mesh_shape::*;

mod hk_memory_mesh_texture;
use hk_memory_mesh_texture::*;

mod hk_memory_mesh_vertex_buffer;
use hk_memory_mesh_vertex_buffer::*;

mod hk_memory_resource_container;
use hk_memory_resource_container::*;

mod hk_memory_resource_handle_external_link;
use hk_memory_resource_handle_external_link::*;

mod hk_memory_resource_handle;
use hk_memory_resource_handle::*;

mod hk_memory_tracker_attribute;
use hk_memory_tracker_attribute::*;

mod hk_mesh_body;
use hk_mesh_body::*;

mod hk_mesh_bone_index_mapping;
use hk_mesh_bone_index_mapping::*;

mod hk_mesh_material;
use hk_mesh_material::*;

mod hk_mesh_section_cinfo;
use hk_mesh_section_cinfo::*;

mod hk_mesh_section;
use hk_mesh_section::*;

mod hk_mesh_shape;
use hk_mesh_shape::*;

mod hk_mesh_texture;
use hk_mesh_texture::*;

mod hk_mesh_vertex_buffer;
use hk_mesh_vertex_buffer::*;

mod hk_modeler_node_type_attribute;
use hk_modeler_node_type_attribute::*;

mod hk_monitor_stream_color_table_color_pair;
use hk_monitor_stream_color_table_color_pair::*;

mod hk_monitor_stream_color_table;
use hk_monitor_stream_color_table::*;

mod hk_monitor_stream_frame_info;
use hk_monitor_stream_frame_info::*;

mod hk_monitor_stream_string_map_string_map;
use hk_monitor_stream_string_map_string_map::*;

mod hk_monitor_stream_string_map;
use hk_monitor_stream_string_map::*;

mod hk_mopp_bv_tree_shape_base;
use hk_mopp_bv_tree_shape_base::*;

mod hk_motion_state;
use hk_motion_state::*;

mod hk_multiple_vertex_buffer_element_info;
use hk_multiple_vertex_buffer_element_info::*;

mod hk_multiple_vertex_buffer_locked_element;
use hk_multiple_vertex_buffer_locked_element::*;

mod hk_multiple_vertex_buffer_vertex_buffer_info;
use hk_multiple_vertex_buffer_vertex_buffer_info::*;

mod hk_multiple_vertex_buffer;
use hk_multiple_vertex_buffer::*;

mod hk_multi_thread_check;
use hk_multi_thread_check::*;

mod hkp_2_d_ang_constraint_atom;
use hkp_2_d_ang_constraint_atom::*;

mod hkp_aabb_phantom;
use hkp_aabb_phantom::*;

mod hk_packed_vector_3;
use hk_packed_vector_3::*;

mod hk_packfile_header;
use hk_packfile_header::*;

mod hk_packfile_section_header;
use hk_packfile_section_header::*;

mod hkp_action;
use hkp_action::*;

mod hkp_agent_1_n_sector;
use hkp_agent_1_n_sector::*;

mod hkp_ang_constraint_atom;
use hkp_ang_constraint_atom::*;

mod hkp_ang_friction_constraint_atom;
use hkp_ang_friction_constraint_atom::*;

mod hkp_ang_limit_constraint_atom;
use hkp_ang_limit_constraint_atom::*;

mod hkp_ang_motor_constraint_atom;
use hkp_ang_motor_constraint_atom::*;

mod hkp_angular_dashpot_action;
use hkp_angular_dashpot_action::*;

mod hkp_array_action;
use hkp_array_action::*;

mod hkp_ball_and_socket_constraint_data_atoms;
use hkp_ball_and_socket_constraint_data_atoms::*;

mod hkp_ball_and_socket_constraint_data;
use hkp_ball_and_socket_constraint_data::*;

mod hkp_ball_gun;
use hkp_ball_gun::*;

mod hkp_ball_socket_chain_data_constraint_info;
use hkp_ball_socket_chain_data_constraint_info::*;

mod hkp_ball_socket_chain_data;
use hkp_ball_socket_chain_data::*;

mod hkp_ball_socket_constraint_atom;
use hkp_ball_socket_constraint_atom::*;

mod hkp_binary_action;
use hkp_binary_action::*;

mod hkp_box_motion;
use hkp_box_motion::*;

mod hkp_box_shape;
use hkp_box_shape::*;

mod hkp_breakable_body;
use hkp_breakable_body::*;

mod hkp_breakable_constraint_data;
use hkp_breakable_constraint_data::*;

mod hkp_bridge_atoms;
use hkp_bridge_atoms::*;

mod hkp_bridge_constraint_atom;
use hkp_bridge_constraint_atom::*;

mod hkp_broad_phase_handle;
use hkp_broad_phase_handle::*;

mod hkp_bv_shape;
use hkp_bv_shape::*;

mod hkp_bv_tree_shape;
use hkp_bv_tree_shape::*;

mod hkp_caching_shape_phantom;
use hkp_caching_shape_phantom::*;

mod hkp_callback_constraint_motor;
use hkp_callback_constraint_motor::*;

mod hkp_capsule_shape;
use hkp_capsule_shape::*;

mod hkp_cd_body;
use hkp_cd_body::*;

mod hkp_center_of_mass_changer_modifier_constraint_atom;
use hkp_center_of_mass_changer_modifier_constraint_atom::*;

mod hkp_character_controller_cinfo;
use hkp_character_controller_cinfo::*;

mod hkp_character_motion;
use hkp_character_motion::*;

mod hkp_character_proxy_cinfo;
use hkp_character_proxy_cinfo::*;

mod hkp_character_rigid_body_cinfo;
use hkp_character_rigid_body_cinfo::*;

mod hkp_cog_wheel_constraint_atom;
use hkp_cog_wheel_constraint_atom::*;

mod hkp_cog_wheel_constraint_data_atoms;
use hkp_cog_wheel_constraint_data_atoms::*;

mod hkp_cog_wheel_constraint_data;
use hkp_cog_wheel_constraint_data::*;

mod hkp_collidable_bounding_volume_data;
use hkp_collidable_bounding_volume_data::*;

mod hkp_collidable_collidable_filter;
use hkp_collidable_collidable_filter::*;

mod hkp_collidable;
use hkp_collidable::*;

mod hkp_collision_filter_list;
use hkp_collision_filter_list::*;

mod hkp_collision_filter;
use hkp_collision_filter::*;

mod hkp_compressed_mesh_shape_big_triangle;
use hkp_compressed_mesh_shape_big_triangle::*;

mod hkp_compressed_mesh_shape_chunk;
use hkp_compressed_mesh_shape_chunk::*;

mod hkp_compressed_mesh_shape_convex_piece;
use hkp_compressed_mesh_shape_convex_piece::*;

mod hkp_compressed_mesh_shape;
use hkp_compressed_mesh_shape::*;

mod hkp_compressed_sampled_height_field_shape;
use hkp_compressed_sampled_height_field_shape::*;

mod hkp_cone_limit_constraint_atom;
use hkp_cone_limit_constraint_atom::*;

mod hkp_constrained_system_filter;
use hkp_constrained_system_filter::*;

mod hkp_constraint_atom;
use hkp_constraint_atom::*;

mod hkp_constraint_chain_data;
use hkp_constraint_chain_data::*;

mod hkp_constraint_chain_instance_action;
use hkp_constraint_chain_instance_action::*;

mod hkp_constraint_chain_instance;
use hkp_constraint_chain_instance::*;

mod hkp_constraint_collision_filter;
use hkp_constraint_collision_filter::*;

mod hkp_constraint_data;
use hkp_constraint_data::*;

mod hkp_constraint_instance_small_array_serialize_override_type;
use hkp_constraint_instance_small_array_serialize_override_type::*;

mod hkp_constraint_instance;
use hkp_constraint_instance::*;

mod hkp_constraint_motor;
use hkp_constraint_motor::*;

mod hkp_convex_list_filter;
use hkp_convex_list_filter::*;

mod hkp_convex_list_shape;
use hkp_convex_list_shape::*;

mod hkp_convex_piece_mesh_shape;
use hkp_convex_piece_mesh_shape::*;

mod hkp_convex_piece_stream_data;
use hkp_convex_piece_stream_data::*;

mod hkp_convex_shape;
use hkp_convex_shape::*;

mod hkp_convex_transform_shape_base;
use hkp_convex_transform_shape_base::*;

mod hkp_convex_transform_shape;
use hkp_convex_transform_shape::*;

mod hkp_convex_translate_shape;
use hkp_convex_translate_shape::*;

mod hkp_convex_vertices_connectivity;
use hkp_convex_vertices_connectivity::*;

mod hkp_convex_vertices_shape_four_vectors;
use hkp_convex_vertices_shape_four_vectors::*;

mod hkp_convex_vertices_shape;
use hkp_convex_vertices_shape::*;

mod hkp_cylinder_shape;
use hkp_cylinder_shape::*;

mod hkp_dashpot_action;
use hkp_dashpot_action::*;

mod hkp_default_convex_list_filter;
use hkp_default_convex_list_filter::*;

mod hkp_default_world_memory_watch_dog;
use hkp_default_world_memory_watch_dog::*;

mod hkp_disable_entity_collision_filter;
use hkp_disable_entity_collision_filter::*;

mod hkp_display_binding_data_physics_system;
use hkp_display_binding_data_physics_system::*;

mod hkp_display_binding_data_rigid_body;
use hkp_display_binding_data_rigid_body::*;

mod hkp_display_binding_data;
use hkp_display_binding_data::*;

mod hkp_entity_extended_listeners;
use hkp_entity_extended_listeners::*;

mod hkp_entity_small_array_serialize_override_type;
use hkp_entity_small_array_serialize_override_type::*;

mod hkp_entity_spu_collision_callback;
use hkp_entity_spu_collision_callback::*;

mod hkp_entity;
use hkp_entity::*;

mod hkp_extended_mesh_shape_shapes_subpart;
use hkp_extended_mesh_shape_shapes_subpart::*;

mod hkp_extended_mesh_shape_subpart;
use hkp_extended_mesh_shape_subpart::*;

mod hkp_extended_mesh_shape_triangles_subpart;
use hkp_extended_mesh_shape_triangles_subpart::*;

mod hkp_extended_mesh_shape;
use hkp_extended_mesh_shape::*;

mod hkp_fast_mesh_shape;
use hkp_fast_mesh_shape::*;

mod hkp_first_person_gun;
use hkp_first_person_gun::*;

mod hkp_fixed_rigid_motion;
use hkp_fixed_rigid_motion::*;

mod hkp_generic_constraint_data_scheme_constraint_info;
use hkp_generic_constraint_data_scheme_constraint_info::*;

mod hkp_generic_constraint_data_scheme;
use hkp_generic_constraint_data_scheme::*;

mod hkp_generic_constraint_data;
use hkp_generic_constraint_data::*;

mod hkp_gravity_gun;
use hkp_gravity_gun::*;

mod hkp_group_collision_filter;
use hkp_group_collision_filter::*;

mod hkp_group_filter;
use hkp_group_filter::*;

mod hkp_height_field_shape;
use hkp_height_field_shape::*;

mod hkp_hinge_constraint_data_atoms;
use hkp_hinge_constraint_data_atoms::*;

mod hkp_hinge_constraint_data;
use hkp_hinge_constraint_data::*;

mod hkp_hinge_limits_data_atoms;
use hkp_hinge_limits_data_atoms::*;

mod hkp_hinge_limits_data;
use hkp_hinge_limits_data::*;

mod hkp_ignore_modifier_constraint_atom;
use hkp_ignore_modifier_constraint_atom::*;

mod hkp_keyframed_rigid_motion;
use hkp_keyframed_rigid_motion::*;

mod hkp_limited_force_constraint_motor;
use hkp_limited_force_constraint_motor::*;

mod hkp_limited_hinge_constraint_data_atoms;
use hkp_limited_hinge_constraint_data_atoms::*;

mod hkp_limited_hinge_constraint_data;
use hkp_limited_hinge_constraint_data::*;

mod hkp_lin_constraint_atom;
use hkp_lin_constraint_atom::*;

mod hkp_linear_parametric_curve;
use hkp_linear_parametric_curve::*;

mod hkp_lin_friction_constraint_atom;
use hkp_lin_friction_constraint_atom::*;

mod hkp_linked_collidable;
use hkp_linked_collidable::*;

mod hkp_lin_limit_constraint_atom;
use hkp_lin_limit_constraint_atom::*;

mod hkp_lin_motor_constraint_atom;
use hkp_lin_motor_constraint_atom::*;

mod hkp_lin_soft_constraint_atom;
use hkp_lin_soft_constraint_atom::*;

mod hkp_list_shape_child_info;
use hkp_list_shape_child_info::*;

mod hkp_list_shape;
use hkp_list_shape::*;

mod hkp_malleable_constraint_data;
use hkp_malleable_constraint_data::*;

mod hkp_mass_changer_modifier_constraint_atom;
use hkp_mass_changer_modifier_constraint_atom::*;

mod hkp_mass_properties;
use hkp_mass_properties::*;

mod hkp_material;
use hkp_material::*;

mod hkp_max_size_motion;
use hkp_max_size_motion::*;

mod hkp_mesh_material;
use hkp_mesh_material::*;

mod hkp_mesh_shape_subpart;
use hkp_mesh_shape_subpart::*;

mod hkp_mesh_shape;
use hkp_mesh_shape::*;

mod hkp_modifier_constraint_atom;
use hkp_modifier_constraint_atom::*;

mod hkp_mopp_bv_tree_shape;
use hkp_mopp_bv_tree_shape::*;

mod hkp_mopp_code_code_info;
use hkp_mopp_code_code_info::*;

mod hkp_mopp_code_reindexed_terminal;
use hkp_mopp_code_reindexed_terminal::*;

mod hkp_mopp_code;
use hkp_mopp_code::*;

mod hkp_motion;
use hkp_motion::*;

mod hkp_motor_action;
use hkp_motor_action::*;

mod hkp_mounted_ball_gun;
use hkp_mounted_ball_gun::*;

mod hkp_mouse_spring_action;
use hkp_mouse_spring_action::*;

mod hkp_moving_surface_modifier_constraint_atom;
use hkp_moving_surface_modifier_constraint_atom::*;

mod hkp_multi_ray_shape_ray;
use hkp_multi_ray_shape_ray::*;

mod hkp_multi_ray_shape;
use hkp_multi_ray_shape::*;

mod hkp_multi_sphere_shape;
use hkp_multi_sphere_shape::*;

mod hkp_multithreaded_vehicle_manager;
use hkp_multithreaded_vehicle_manager::*;

mod hkp_named_mesh_material;
use hkp_named_mesh_material::*;

mod hkp_null_collision_filter;
use hkp_null_collision_filter::*;

mod hk_post_finish_attribute;
use hk_post_finish_attribute::*;

mod hkp_overwrite_pivot_constraint_atom;
use hkp_overwrite_pivot_constraint_atom::*;

mod hkp_pair_collision_filter_map_pair_filter_key_override_type;
use hkp_pair_collision_filter_map_pair_filter_key_override_type::*;

mod hkp_pair_collision_filter;
use hkp_pair_collision_filter::*;

mod hkp_parametric_curve;
use hkp_parametric_curve::*;

mod hkp_phantom_callback_shape;
use hkp_phantom_callback_shape::*;

mod hkp_phantom;
use hkp_phantom::*;

mod hkp_physics_data;
use hkp_physics_data::*;

mod hkp_physics_system_with_contacts;
use hkp_physics_system_with_contacts::*;

mod hkp_physics_system;
use hkp_physics_system::*;

mod hkp_plane_shape;
use hkp_plane_shape::*;

mod hkp_point_to_path_constraint_data;
use hkp_point_to_path_constraint_data::*;

mod hkp_point_to_plane_constraint_data_atoms;
use hkp_point_to_plane_constraint_data_atoms::*;

mod hkp_point_to_plane_constraint_data;
use hkp_point_to_plane_constraint_data::*;

mod hkp_position_constraint_motor;
use hkp_position_constraint_motor::*;

mod hkp_powered_chain_data_constraint_info;
use hkp_powered_chain_data_constraint_info::*;

mod hkp_powered_chain_data;
use hkp_powered_chain_data::*;

mod hkp_powered_chain_mapper_link_info;
use hkp_powered_chain_mapper_link_info::*;

mod hkp_powered_chain_mapper_target;
use hkp_powered_chain_mapper_target::*;

mod hkp_powered_chain_mapper;
use hkp_powered_chain_mapper::*;

mod hkp_prismatic_constraint_data_atoms;
use hkp_prismatic_constraint_data_atoms::*;

mod hkp_prismatic_constraint_data;
use hkp_prismatic_constraint_data::*;

mod hkp_projectile_gun;
use hkp_projectile_gun::*;

mod hkp_property_value;
use hkp_property_value::*;

mod hkp_property;
use hkp_property::*;

mod hkp_pulley_constraint_atom;
use hkp_pulley_constraint_atom::*;

mod hkp_pulley_constraint_data_atoms;
use hkp_pulley_constraint_data_atoms::*;

mod hkp_pulley_constraint_data;
use hkp_pulley_constraint_data::*;

mod hkp_rack_and_pinion_constraint_atom;
use hkp_rack_and_pinion_constraint_atom::*;

mod hkp_rack_and_pinion_constraint_data_atoms;
use hkp_rack_and_pinion_constraint_data_atoms::*;

mod hkp_rack_and_pinion_constraint_data;
use hkp_rack_and_pinion_constraint_data::*;

mod hkp_ragdoll_constraint_data_atoms;
use hkp_ragdoll_constraint_data_atoms::*;

mod hkp_ragdoll_constraint_data;
use hkp_ragdoll_constraint_data::*;

mod hkp_ragdoll_limits_data_atoms;
use hkp_ragdoll_limits_data_atoms::*;

mod hkp_ragdoll_limits_data;
use hkp_ragdoll_limits_data::*;

mod hkp_ragdoll_motor_constraint_atom;
use hkp_ragdoll_motor_constraint_atom::*;

mod hkp_ray_collidable_filter;
use hkp_ray_collidable_filter::*;

mod hkp_ray_shape_collection_filter;
use hkp_ray_shape_collection_filter::*;

mod hkp_reject_chassis_listener;
use hkp_reject_chassis_listener::*;

mod hkp_remove_terminals_mopp_modifier;
use hkp_remove_terminals_mopp_modifier::*;

mod hkp_reorient_action;
use hkp_reorient_action::*;

mod hkp_rigid_body;
use hkp_rigid_body::*;

mod hkp_rotational_constraint_data_atoms;
use hkp_rotational_constraint_data_atoms::*;

mod hkp_rotational_constraint_data;
use hkp_rotational_constraint_data::*;

mod hkp_sampled_height_field_shape;
use hkp_sampled_height_field_shape::*;

mod hkp_serialized_agent_nn_entry;
use hkp_serialized_agent_nn_entry::*;

mod hkp_serialized_display_marker_list;
use hkp_serialized_display_marker_list::*;

mod hkp_serialized_display_marker;
use hkp_serialized_display_marker::*;

mod hkp_serialized_display_rb_transforms_display_transform_pair;
use hkp_serialized_display_rb_transforms_display_transform_pair::*;

mod hkp_serialized_display_rb_transforms;
use hkp_serialized_display_rb_transforms::*;

mod hkp_serialized_sub_track_1_n_info;
use hkp_serialized_sub_track_1_n_info::*;

mod hkp_serialized_track_1_n_info;
use hkp_serialized_track_1_n_info::*;

mod hkp_set_local_rotations_constraint_atom;
use hkp_set_local_rotations_constraint_atom::*;

mod hkp_set_local_transforms_constraint_atom;
use hkp_set_local_transforms_constraint_atom::*;

mod hkp_set_local_translations_constraint_atom;
use hkp_set_local_translations_constraint_atom::*;

mod hkp_setup_stabilization_atom;
use hkp_setup_stabilization_atom::*;

mod hkp_shape_collection_filter;
use hkp_shape_collection_filter::*;

mod hkp_shape_collection;
use hkp_shape_collection::*;

mod hkp_shape_container;
use hkp_shape_container::*;

mod hkp_shape_info;
use hkp_shape_info::*;

mod hkp_shape_modifier;
use hkp_shape_modifier::*;

mod hkp_shape_phantom;
use hkp_shape_phantom::*;

mod hkp_shape;
use hkp_shape::*;

mod hkp_simple_contact_constraint_atom;
use hkp_simple_contact_constraint_atom::*;

mod hkp_simple_contact_constraint_data_info;
use hkp_simple_contact_constraint_data_info::*;

mod hkp_simple_mesh_shape_triangle;
use hkp_simple_mesh_shape_triangle::*;

mod hkp_simple_mesh_shape;
use hkp_simple_mesh_shape::*;

mod hkp_simple_shape_phantom_collision_detail;
use hkp_simple_shape_phantom_collision_detail::*;

mod hkp_simple_shape_phantom;
use hkp_simple_shape_phantom::*;

mod hkp_simulation;
use hkp_simulation::*;

mod hkp_single_shape_container;
use hkp_single_shape_container::*;

mod hkp_soft_contact_modifier_constraint_atom;
use hkp_soft_contact_modifier_constraint_atom::*;

mod hkp_sphere_motion;
use hkp_sphere_motion::*;

mod hkp_sphere_rep_shape;
use hkp_sphere_rep_shape::*;

mod hkp_sphere_shape;
use hkp_sphere_shape::*;

mod hkp_spring_action;
use hkp_spring_action::*;

mod hkp_spring_damper_constraint_motor;
use hkp_spring_damper_constraint_motor::*;

mod hkp_stiff_spring_chain_data_constraint_info;
use hkp_stiff_spring_chain_data_constraint_info::*;

mod hkp_stiff_spring_chain_data;
use hkp_stiff_spring_chain_data::*;

mod hkp_stiff_spring_constraint_atom;
use hkp_stiff_spring_constraint_atom::*;

mod hkp_stiff_spring_constraint_data_atoms;
use hkp_stiff_spring_constraint_data_atoms::*;

mod hkp_stiff_spring_constraint_data;
use hkp_stiff_spring_constraint_data::*;

mod hkp_storage_extended_mesh_shape_material;
use hkp_storage_extended_mesh_shape_material::*;

mod hkp_storage_extended_mesh_shape_mesh_subpart_storage;
use hkp_storage_extended_mesh_shape_mesh_subpart_storage::*;

mod hkp_storage_extended_mesh_shape_shape_subpart_storage;
use hkp_storage_extended_mesh_shape_shape_subpart_storage::*;

mod hkp_storage_extended_mesh_shape;
use hkp_storage_extended_mesh_shape::*;

mod hkp_storage_mesh_shape_subpart_storage;
use hkp_storage_mesh_shape_subpart_storage::*;

mod hkp_storage_mesh_shape;
use hkp_storage_mesh_shape::*;

mod hkp_storage_sampled_height_field_shape;
use hkp_storage_sampled_height_field_shape::*;

mod hkp_thin_box_motion;
use hkp_thin_box_motion::*;

mod hkp_transform_shape;
use hkp_transform_shape::*;

mod hkp_triangle_shape;
use hkp_triangle_shape::*;

mod hkp_trigger_volume_event_info;
use hkp_trigger_volume_event_info::*;

mod hkp_trigger_volume;
use hkp_trigger_volume::*;

mod hkp_tri_sampled_height_field_bv_tree_shape;
use hkp_tri_sampled_height_field_bv_tree_shape::*;

mod hkp_tri_sampled_height_field_collection;
use hkp_tri_sampled_height_field_collection::*;

mod hkp_twist_limit_constraint_atom;
use hkp_twist_limit_constraint_atom::*;

mod hkp_typed_broad_phase_handle;
use hkp_typed_broad_phase_handle::*;

mod hkp_tyremark_point;
use hkp_tyremark_point::*;

mod hkp_tyremarks_info;
use hkp_tyremarks_info::*;

mod hkp_tyremarks_wheel;
use hkp_tyremarks_wheel::*;

mod hkp_unary_action;
use hkp_unary_action::*;

mod hkp_vehicle_aerodynamics;
use hkp_vehicle_aerodynamics::*;

mod hkp_vehicle_brake;
use hkp_vehicle_brake::*;

mod hkp_vehicle_cast_batching_manager;
use hkp_vehicle_cast_batching_manager::*;

mod hkp_vehicle_data_wheel_component_params;
use hkp_vehicle_data_wheel_component_params::*;

mod hkp_vehicle_data;
use hkp_vehicle_data::*;

mod hkp_vehicle_default_aerodynamics;
use hkp_vehicle_default_aerodynamics::*;

mod hkp_vehicle_default_analog_driver_input;
use hkp_vehicle_default_analog_driver_input::*;

mod hkp_vehicle_default_brake_wheel_braking_properties;
use hkp_vehicle_default_brake_wheel_braking_properties::*;

mod hkp_vehicle_default_brake;
use hkp_vehicle_default_brake::*;

mod hkp_vehicle_default_engine;
use hkp_vehicle_default_engine::*;

mod hkp_vehicle_default_steering;
use hkp_vehicle_default_steering::*;

mod hkp_vehicle_default_suspension_wheel_spring_suspension_parameters;
use hkp_vehicle_default_suspension_wheel_spring_suspension_parameters::*;

mod hkp_vehicle_default_suspension;
use hkp_vehicle_default_suspension::*;

mod hkp_vehicle_default_transmission;
use hkp_vehicle_default_transmission::*;

mod hkp_vehicle_default_velocity_damper;
use hkp_vehicle_default_velocity_damper::*;

mod hkp_vehicle_driver_input_analog_status;
use hkp_vehicle_driver_input_analog_status::*;

mod hkp_vehicle_driver_input_status;
use hkp_vehicle_driver_input_status::*;

mod hkp_vehicle_driver_input;
use hkp_vehicle_driver_input::*;

mod hkp_vehicle_engine;
use hkp_vehicle_engine::*;

mod hkp_vehicle_friction_description_axis_description;
use hkp_vehicle_friction_description_axis_description::*;

mod hkp_vehicle_friction_description;
use hkp_vehicle_friction_description::*;

mod hkp_vehicle_friction_status_axis_status;
use hkp_vehicle_friction_status_axis_status::*;

mod hkp_vehicle_friction_status;
use hkp_vehicle_friction_status::*;

mod hkp_vehicle_instance_wheel_info;
use hkp_vehicle_instance_wheel_info::*;

mod hkp_vehicle_instance;
use hkp_vehicle_instance::*;

mod hkp_vehicle_linear_cast_batching_manager;
use hkp_vehicle_linear_cast_batching_manager::*;

mod hkp_vehicle_linear_cast_wheel_collide_wheel_state;
use hkp_vehicle_linear_cast_wheel_collide_wheel_state::*;

mod hkp_vehicle_linear_cast_wheel_collide;
use hkp_vehicle_linear_cast_wheel_collide::*;

mod hkp_vehicle_manager;
use hkp_vehicle_manager::*;

mod hkp_vehicle_ray_cast_batching_manager;
use hkp_vehicle_ray_cast_batching_manager::*;

mod hkp_vehicle_ray_cast_wheel_collide;
use hkp_vehicle_ray_cast_wheel_collide::*;

mod hkp_vehicle_steering;
use hkp_vehicle_steering::*;

mod hkp_vehicle_suspension_suspension_wheel_parameters;
use hkp_vehicle_suspension_suspension_wheel_parameters::*;

mod hkp_vehicle_suspension;
use hkp_vehicle_suspension::*;

mod hkp_vehicle_transmission;
use hkp_vehicle_transmission::*;

mod hkp_vehicle_velocity_damper;
use hkp_vehicle_velocity_damper::*;

mod hkp_vehicle_wheel_collide;
use hkp_vehicle_wheel_collide::*;

mod hkp_velocity_constraint_motor;
use hkp_velocity_constraint_motor::*;

mod hkp_viscous_surface_modifier_constraint_atom;
use hkp_viscous_surface_modifier_constraint_atom::*;

mod hkp_welding_utility;
use hkp_welding_utility::*;

mod hkp_wheel_constraint_data_atoms;
use hkp_wheel_constraint_data_atoms::*;

mod hkp_wheel_constraint_data;
use hkp_wheel_constraint_data::*;

mod hkp_world_cinfo;
use hkp_world_cinfo::*;

mod hkp_world_object;
use hkp_world_object::*;

mod hkp_world;
use hkp_world::*;

mod hk_q_transform;
use hk_q_transform::*;

mod hk_range_int_32_attribute;
use hk_range_int_32_attribute::*;

mod hk_range_real_attribute;
use hk_range_real_attribute::*;

mod hk_referenced_object;
use hk_referenced_object::*;

mod hk_reflected_file_attribute;
use hk_reflected_file_attribute::*;

mod hk_resource_base;
use hk_resource_base::*;

mod hk_resource_container;
use hk_resource_container::*;

mod hk_resource_handle;
use hk_resource_handle::*;

mod hk_root_level_container_named_variant;
use hk_root_level_container_named_variant::*;

mod hk_root_level_container;
use hk_root_level_container::*;

mod hk_semantics_attribute;
use hk_semantics_attribute::*;

mod hk_simple_local_frame;
use hk_simple_local_frame::*;

mod hk_sphere;
use hk_sphere::*;

mod hk_swept_transform;
use hk_swept_transform::*;

mod hk_trace_stream_title;
use hk_trace_stream_title::*;

mod hk_tracker_serializable_scan_snapshot_allocation;
use hk_tracker_serializable_scan_snapshot_allocation::*;

mod hk_tracker_serializable_scan_snapshot_block;
use hk_tracker_serializable_scan_snapshot_block::*;

mod hk_tracker_serializable_scan_snapshot;
use hk_tracker_serializable_scan_snapshot::*;

mod hk_ui_attribute;
use hk_ui_attribute::*;

mod hk_vertex_format_element;
use hk_vertex_format_element::*;

mod hk_vertex_format;
use hk_vertex_format::*;

mod hk_world_memory_available_watch_dog;
use hk_world_memory_available_watch_dog::*;

mod hkx_animated_float;
use hkx_animated_float::*;

mod hkx_animated_matrix;
use hkx_animated_matrix::*;

mod hkx_animated_quaternion;
use hkx_animated_quaternion::*;

mod hkx_animated_vector;
use hkx_animated_vector::*;

mod hkx_attribute_group;
use hkx_attribute_group::*;

mod hkx_attribute_holder;
use hkx_attribute_holder::*;

mod hkx_attribute;
use hkx_attribute::*;

mod hkx_camera;
use hkx_camera::*;

mod hkx_edge_selection_channel;
use hkx_edge_selection_channel::*;

mod hkx_enum_item;
use hkx_enum_item::*;

mod hkx_enum;
use hkx_enum::*;

mod hkx_environment_variable;
use hkx_environment_variable::*;

mod hkx_environment;
use hkx_environment::*;

mod hkx_index_buffer;
use hkx_index_buffer::*;

mod hkx_light;
use hkx_light::*;

mod hkx_material_effect;
use hkx_material_effect::*;

mod hkx_material_property;
use hkx_material_property::*;

mod hkx_material_shader_set;
use hkx_material_shader_set::*;

mod hkx_material_shader;
use hkx_material_shader::*;

mod hkx_material_texture_stage;
use hkx_material_texture_stage::*;

mod hkx_material;
use hkx_material::*;

mod hkx_mesh_section;
use hkx_mesh_section::*;

mod hkx_mesh_user_channel_info;
use hkx_mesh_user_channel_info::*;

mod hkx_mesh;
use hkx_mesh::*;

mod hkx_node_annotation_data;
use hkx_node_annotation_data::*;

mod hkx_node_selection_set;
use hkx_node_selection_set::*;

mod hkx_node;
use hkx_node::*;

mod hkx_scene;
use hkx_scene::*;

mod hkx_skin_binding;
use hkx_skin_binding::*;

mod hkx_sparsely_animated_bool;
use hkx_sparsely_animated_bool::*;

mod hkx_sparsely_animated_enum;
use hkx_sparsely_animated_enum::*;

mod hkx_sparsely_animated_int;
use hkx_sparsely_animated_int::*;

mod hkx_sparsely_animated_string;
use hkx_sparsely_animated_string::*;

mod hkx_texture_file;
use hkx_texture_file::*;

mod hkx_texture_inplace;
use hkx_texture_inplace::*;

mod hkx_triangle_selection_channel;
use hkx_triangle_selection_channel::*;

mod hkx_vertex_buffer_vertex_data;
use hkx_vertex_buffer_vertex_data::*;

mod hkx_vertex_buffer;
use hkx_vertex_buffer::*;

mod hkx_vertex_description_element_decl;
use hkx_vertex_description_element_decl::*;

mod hkx_vertex_description;
use hkx_vertex_description::*;

mod hkx_vertex_float_data_channel;
use hkx_vertex_float_data_channel::*;

mod hkx_vertex_int_data_channel;
use hkx_vertex_int_data_channel::*;

mod hkx_vertex_selection_channel;
use hkx_vertex_selection_channel::*;

mod hkx_vertex_vector_data_channel;
use hkx_vertex_vector_data_channel::*;
