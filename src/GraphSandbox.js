import Graph from "react-graph-vis";

const GraphSandbox = () => {
    const graph = {
        nodes: [
            { id: 1, label: 'A' },
            { id: 2, label: 'B' },
        ],
        edges: [
            { from: 1, to: 2, weight: 5 },
        ],
    };

    const options = {
        layout: {
            hierarchical: false
        },
        edges: {
            color: "#666666",
            arrows: {
                to: { enabled: false }
            },
            smooth: {
                type: "continuous"
            },
            label: {
                enabled: true,
                color: "black",
                background: "transparent",
                font: {
                    size: 12
                },
                smooth: {
                    type: "dynamic"
                },
            }
        },
        nodes: {
            shape: 'circle',
            color: 'white'
        }
    };

    const events = {
        select: function (event) {
            var { nodes, edges } = event;
            console.log(edges);
            console.log(nodes);
        }
    };


    return (
        <div className="w-full h-full">
            <Graph
                graph={graph}
                options={options}
                events={events}
                getNetwork={(network) => { }}
                style={{
                    height: '100%',
                    borderBottomRightRadius: '0.375rem',
                    borderBottomLeftRadius: '0.375rem',
                }}
            />
        </div>
    );
}

export default GraphSandbox;