import './App.css'
import GraphSandbox from './GraphSandbox'
import React, { useState, useRef } from 'react'
import { Snackbar, IconButton } from '@material-ui/core'
import { isEmptyObject } from 'jquery'
import CloseIcon from '@mui/icons-material/Close'
import { invoke } from '@tauri-apps/api/tauri'

const moment = require('moment-js')
// document.addEventListener('contextmenu', event => event.preventDefault());

function App() {
  const [simulating, setSimulating] = useState(false)

  const [snackbarMessage, setSnackbarMessage] = useState('CUSTOM_SNACKBAR_MESSAGE')
  const [openSnackbar, setOpenSnackbar] = useState(false)

  const [loggerMessages, setLoggerMessages] = useState([])

  const graphSandboxRef = useRef(null)

  const [nodes, setNodes] = useState([])

  const [newNodeName, setNewNodeName] = useState('')
  const [isGraphEmpty, setIsGraphEmpty] = useState(true)

  const [selectedNode, setSelectedNode] = useState(null)
  const [selectedEdge, setSelectedEdge] = useState(null)

  const [addEdgeNode1, setEdgeNode1] = useState(-1)
  const [addEdgeNode2, setEdgeNode2] = useState(-1)
  const [simulationNode, setSimulationNode] = useState(-1)

  const [connectionValue, setConnectionValue] = useState(0)

  const handleNewNodeNameChange = (event) => {
    setNewNodeName(event.target.value)
  }

  const handleNewConnValueChange = (event) => {
    setConnectionValue(event.target.value)
  }

  const handleAddEdgeNode1Change = (event) => {
    setEdgeNode1(event.target.value)
  }

  const handleAddEdgeNode2Change = (event) => {
    setEdgeNode2(event.target.value)
  }

  const handleSimulatingNodeChange = (event) => {
    setSimulationNode(event.target.value)
  }

  const checkForEmptyGraph = async () => {
    return new Promise((resolve) => {
      setTimeout(() => {
        if (graphSandboxRef.current) {
          const isEmpty = graphSandboxRef.current.isEmptyGraph()
          resolve(isEmpty)
        } else {
          resolve(true)
        }
      }, 100)
    })
  }

  const handleSelectedNodeChange = (selectedNode) => {
    setSelectedNode(selectedNode)
  }

  const handleSelectedEdgeChange = (selectedEdge) => {
    setSelectedEdge(selectedEdge)
  }

  const emptyGraph = async () => {
    if (graphSandboxRef.current) {
      graphSandboxRef.current.emptyGraph()

      let state = await checkForEmptyGraph()
      setIsGraphEmpty(state)

      setSelectedNode(null)
      setSelectedEdge(null)

      setEdgeNode1(-1)
      setEdgeNode2(-1)
      setSimulationNode(-1)
      setConnectionValue(0)

      const updatedNodes = graphSandboxRef.current.getNodes()
      setNodes(updatedNodes)
    }
  }

  const handleAddNewNodeClick = async () => {
    if (graphSandboxRef.current) {
      if (!graphSandboxRef.current.hasNode(newNodeName)) {
        graphSandboxRef.current.addNode(newNodeName)
        setNewNodeName('')
        let state = await checkForEmptyGraph()
        setIsGraphEmpty(state)

        const updatedNodes = graphSandboxRef.current.getNodes()
        setNodes(updatedNodes)
      } else {
        handleOpenSnackbar("Esse nó já existe!")
      }
    }
  }

  const handleAddNewEdgeClick = async () => {
    if (graphSandboxRef.current) {
      if (!graphSandboxRef.current.hasEdge(addEdgeNode1, addEdgeNode2)) {
        graphSandboxRef.current.addEdge(addEdgeNode1, addEdgeNode2, Number(connectionValue))

        setEdgeNode1(-1)
        setEdgeNode2(-1)
        setConnectionValue(0)

        let state = await checkForEmptyGraph()
        setIsGraphEmpty(state)
      } else {
        handleOpenSnackbar("Essa ligação já existe!")
      }
    }
  }

  const handleDeleteSelectedNodeClick = async () => {
    if (graphSandboxRef.current) {
      graphSandboxRef.current.removeNode(selectedNode)
      setSelectedNode(undefined)
      let state = await checkForEmptyGraph()
      setIsGraphEmpty(state)

      setEdgeNode1(-1)
      setEdgeNode2(-1)
      setSimulationNode(-1)
      setConnectionValue(0)

      const updatedNodes = graphSandboxRef.current.getNodes()
      setNodes(updatedNodes)
    }
  }

  const handleDeleteSelectedEdgeClick = () => {
    if (graphSandboxRef.current) {
      graphSandboxRef.current.removeEdge(selectedEdge)
      setSelectedEdge(undefined)
    }
  }

  const handleOpenSnackbar = (message) => {
    setSnackbarMessage(message)
    setOpenSnackbar(true)
  }

  const handleCloseSnackbar = () => {
    setOpenSnackbar(false)
  }

  const handleSimulating = () => {
    setSimulating(true)
    let graph = graphSandboxRef.current.getGraph()

    attachMessageToLogger('A iniciar o algorítmo \'Nearest Neighbours\' para o grafo criado.')
    attachMessageToLogger(`Número de nós: ${graph.nodes.length}.`)
    attachMessageToLogger(`Número de ligações: ${graph.edges.length}.`)
    attachMessageToLogger(`Nó selecionado de partida/chegada: ${simulationNode}.`)
    attachMessageToLogger('A iniciar comunicação com a API do backend...')

    graph.starting_node = simulationNode

    // a enviar o grafo para o rust
    invoke('start_resolving_tsp', { invoke_message: JSON.stringify(graph) })
  }

  const attachMessageToLogger = (message) => {
    setLoggerMessages((prevMessages) => [...prevMessages, formatLoggerMessage(message)])
  }

  const formatLoggerMessage = (message) => {
    return `[${moment(new Date()).format('DD-MM-YYYY HH:mm:ss')}]: ${message}`
  }

  const action = (
    <React.Fragment>
      <IconButton
        size="small"
        aria-label="close"
        color="inherit"
        onClick={handleCloseSnackbar}
      >
        <CloseIcon fontSize="small" />
      </IconButton>
    </React.Fragment>
  )

  return (
    <div className='AppContainer'>
      <div data-attr="base-layer" className='flex-1 shrink-0 w-full grid grid-cols-8 grid-rows-1 gap-2 p-4'>
        <div data-attr="btn-action-container" className='row-span-1 col-span-2 rounded-md border border-gray-500 flex flex-col'>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Adicionar nó</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <input disabled={simulating} type="text" value={newNodeName} onChange={handleNewNodeNameChange} id="node_identifier" className="border text-xs placeholder:text-xs rounded-sm block w-full p-1.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-gray-200 focus:ring-blue-500 focus:border-blue-500" placeholder="Identificador do nó..." ></input>
            <button disabled={isEmptyObject(newNodeName) || simulating} onClick={handleAddNewNodeClick} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> Adicionar nó </button>
          </div>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Adicionar ligação</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <select
              disabled={simulating}
              defaultValue={-1}
              value={addEdgeNode1}
              onChange={handleAddEdgeNode1Change}
              className="uppercase border text-xs rounded-sm block w-full p-1.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-gray-200 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value={-1} disabled>
                Seleção de nó (1)
              </option>
              {nodes.map((node) => (
                <option key={node.id} value={node.id}>
                  {node.id}
                </option>
              ))}
            </select>
            <select
              disabled={simulating}
              defaultValue={-1}
              value={addEdgeNode2}
              onChange={handleAddEdgeNode2Change}
              className="uppercase border text-xs rounded-sm block w-full p-1.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-gray-200 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value={-1} disabled>
                Seleção de nó (2)
              </option>
              {nodes.map((node) => (
                <option key={node.id} value={node.id}>
                  {node.id}
                </option>
              ))}
            </select>
            <input disabled={simulating} value={connectionValue} onChange={handleNewConnValueChange} type="number" id="connection_value" className="border text-xs placeholder:text-xs rounded-sm block w-full p-1.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-gray-200 focus:ring-blue-500 focus:border-blue-500" placeholder="Peso da ligação..." ></input>
            <button disabled={addEdgeNode1 === -1 || addEdgeNode1 === -1 || isEmptyObject(connectionValue) || simulating} onClick={handleAddNewEdgeClick} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> Adicionar Ligação </button>
          </div>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Remover nós e arestas</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <button disabled={isEmptyObject(selectedNode) || simulating} onClick={handleDeleteSelectedNodeClick} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> Remover nó </button>
            <button disabled={isEmptyObject(selectedEdge) || simulating} onClick={handleDeleteSelectedEdgeClick} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> Remover aresta </button>
            <button disabled={isGraphEmpty || simulating} onClick={emptyGraph} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> Limpar sandbox </button>
          </div>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Opções de simulação</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <select
              disabled={simulating}
              defaultValue={-1}
              value={simulationNode}
              onChange={handleSimulatingNodeChange}
              className="uppercase border text-xs rounded-sm block w-full p-1.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-gray-200 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value={-1} disabled>
                Nó de simulação
              </option>
              {nodes.map((node) => (
                <option key={node.id} value={node.id}>
                  {node.id}
                </option>
              ))}
            </select>
            <button disabled={simulationNode === -1 || simulating} onClick={handleSimulating} className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> simular </button>
          </div>
          <div className='flex-1 p-[0.10em] flex flex-col items-center justify-end'>
            <p className='text-[0.55em] text-gray-300 px-1.5'>NEAREST NEIGHBOURS - v0.1</p>
          </div>
        </div>
        <div data-attr="sandbox-container" className='row-span-1 col-span-6 grid grid-rows-6 gap-2'>
          <div data-attr="sandbox" className='col-span-1 row-span-5 rounded-md border border-gray-500 flex flex-col'>
            <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>sandbox</p>
            <div className='flex-1 w-full bg-gray-200 rounded-b-md'>
              <GraphSandbox
                ref={graphSandboxRef}
                onSelectedNodeChange={handleSelectedNodeChange}
                onSelectedEdgeChange={handleSelectedEdgeChange}
              ></GraphSandbox>
            </div>
          </div>
          <div data-attr="output-container" className='col-span-1 row-span-1 rounded-md border border-gray-500 flex flex-col'>
            <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Output</p>
            <div data-attr="console-logger" className=' h-[61.2px] overflow-y-auto m-1'>
              {loggerMessages.map((message, index) => (
                <p className='text-white text-[0.55em] uppercase w-full p-[0.05rem]' key={index}>{message}</p>
              ))}
            </div>
          </div>
        </div>
      </div>
      <Snackbar
        open={openSnackbar}
        message={snackbarMessage}
        action={action}
        onClose={handleCloseSnackbar}
        autoHideDuration={3000}
      />
    </div>
  )
}

export default App