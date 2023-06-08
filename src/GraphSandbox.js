import React, { useState, useEffect, forwardRef, useImperativeHandle } from 'react'
import Graph from "react-graph-vis"

const GraphSandbox = forwardRef((props, ref) => {
    const [graph, setGraph] = useState({
        nodes: [],
        edges: [],
    })

    const [selectedNode] = useState(null)
    const [selectedEdge] = useState(null)
    const [isGraphEmpty, setIsGraphEmpty] = useState(true)

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
        },
        nodes: {
            shape: 'circle',
            color: 'white'
        }
    }

    const addNode = (identifier) => {
        const newNode = { id: identifier, label: identifier }
        setGraph((prevGraph) => ({
            ...prevGraph,
            nodes: [...prevGraph.nodes, newNode],
        }))
    }

    const addEdge = (node1, node2, weight) => {
        const newEdge = { from: node1, to: node2, weight: weight, label: weight.toString() }
        setGraph((prevGraph) => ({
            ...prevGraph,
            edges: [...prevGraph.edges, newEdge],
        }))
    }

    const removeNode = (nodeId) => {
        setGraph((prevGraph) => {
            const updatedNodes = prevGraph.nodes.filter((node) => node.id !== nodeId)

            const updatedEdges = prevGraph.edges.filter(
                (edge) => edge.from !== nodeId && edge.to !== nodeId
            )

            return {
                nodes: updatedNodes,
                edges: updatedEdges,
            }
        })
    }

    const removeEdge = (edgeId) => {
        setGraph((prevGraph) => {
            const updatedEdges = prevGraph.edges.filter((edge) => edge.id !== edgeId)

            return {
                nodes: prevGraph.nodes,
                edges: updatedEdges,
            }
        });
    }

    const emptyGraph = () => {
        setGraph({
            nodes: [],
            edges: [],
        })
    }

    const getSelectedNode = () => {
        return selectedNode
    }

    const getSelectedEdge = () => {
        return selectedEdge
    }

    const getNodes = () => {
        return graph.nodes
    }

    const getEdges = () => {
        return graph.edges
    }

    const hasNode = (nodeName) => {
        return graph.nodes.some((node) => node.id === nodeName)
    }

    const hasEdge = (node1, node2) => {
        return graph.edges.some(
            (edge) =>
                (edge.from === node1 && edge.to === node2) ||
                (edge.from === node2 && edge.to === node1)
        )
    }

    const handleNodeSelection = ({ nodes }) => {
        const selectedNodeId = nodes[0]
        props.onSelectedNodeChange(selectedNodeId)
    }

    const handleEdgeSelection = ({ edges }) => {
        const selectedEdgeId = edges[0]
        props.onSelectedEdgeChange(selectedEdgeId)
    }

    const handleNodeDeselection = () => {
        props.onSelectedNodeChange(null)
    }

    const handleEdgeDeselection = () => {
        props.onSelectedEdgeChange(null)
    }

    const events = {
        selectNode: handleNodeSelection,
        selectEdge: handleEdgeSelection,
        deselectNode: handleNodeDeselection,
        deselectEdge: handleEdgeDeselection,
    }

    const isEmptyGraph = () => {
        return isGraphEmpty
    }

    const getGraph = () => {
        return graph
    }

    useImperativeHandle(ref, () => ({
        addNode,
        addEdge,
        getNodes,
        getEdges,
        hasNode,
        hasEdge,
        getGraph,
        emptyGraph,
        isEmptyGraph,
        removeNode,
        removeEdge,
        getSelectedNode,
        getSelectedEdge,
    }))

    useEffect(() => {
        setGraph({
            nodes: [],
            edges: [],
        })
    }, [])

    useEffect(() => {
        setIsGraphEmpty(graph.nodes.length === 0 && graph.edges.length === 0)
    }, [graph])

    return (
        <div className="w-full h-full">
            <Graph
                graph={graph}
                options={options}
                events={events}
                style={{
                    height: '100%',
                    borderBottomRightRadius: '0.375rem',
                    borderBottomLeftRadius: '0.375rem',
                }}
            />
        </div>
    )
})

export default GraphSandbox