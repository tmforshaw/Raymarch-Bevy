#define_import_path ray_marching::octree

const MAX_NODES: u32 = 100;

@group(2) @binding(3)
var<storage> serial_octree: array<u32>;

@group(2) @binding(4)
var<uniform> serial_len: u32;

struct Node {
    children: array<u32, 8>,
    data: vec4<f32>,
}

struct Octree {
    root: Node,
    nodes: array<Node, MAX_NODES>,
}

fn get_nth_node(oct: Octree, index: u32) -> Node {
    return oct.nodes[index];
}

fn deserialise() -> vec4<f32> {
    var nodes = array<Node, MAX_NODES>();
    
    for (var i = 0u; i < serial_len; i+=4u) {
        let lower_half = serial_octree[i];
        let lower_middle_half = serial_octree[i + 1];
        let upper_middle_half = serial_octree[i + 2];
        let upper_half = serial_octree[i + 3];

        let child_7 = lower_half & 0xFFF;
        let child_6 = (lower_half >> 12) & 0xFFF;

        let child_5 = ((lower_half >> 24) & 0xFF) | ((lower_middle_half & 0xF) << 4);

        let child_4 = (lower_middle_half >> 4) & 0xFFF;
        let child_3 = (lower_middle_half >> 16) & 0xFFF;

        let child_2 = ((lower_middle_half >> 28) & 0xF) | ((upper_middle_half & 0xFF) << 8);

        let child_1 = (upper_middle_half >> 8) & 0xFFF;
        let child_0 = (upper_middle_half >> 20) & 0xFFF;

        let blue = f32(upper_half & 0xFF) / 255.;
        let green = f32((upper_half >> 8) & 0xFF) / 255.;
        let red = f32((upper_half >> 16) & 0xFF) / 255.;
        let extra = f32((upper_half >> 24) & 0xFF) / 255.;

        nodes[i / 4] = Node(array<u32, 8>(child_0, child_1, child_2, child_3, child_4, child_5, child_6, child_7), vec4<f32>(red, green, blue, extra)); 
    }

    // return Octree(nodes[0], nodes);

    return nodes[nodes[0].children[7]].data;
}
