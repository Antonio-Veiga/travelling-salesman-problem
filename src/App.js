import './App.css';
import GraphSandbox from './GraphSandbox';

function App() {
  return (
    <div className='AppContainer'>
      <div data-attr="base-layer" className='flex-1 shrink-0 w-full grid grid-cols-8 grid-rows-1 gap-2 p-4'>
        <div data-attr="btn-action-container" className='row-span-1 col-span-2 rounded-md border border-gray-500 flex flex-col'>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Adicionar nó</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <input type="text" id="first_name" class="border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500" placeholder="Digite o identificador do nó..." ></input>
            <button className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white'> Adicionar nó </button>
          </div>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Adicionar nó</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <button className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase 
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white'> Remover nó </button>
          </div>
          <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Adicionar nó</p>
          <div className='h-fit shirnk-0 p-3 flex flex-col items-center justify-start gap-2 border-b border-gray-500'>
            <button disabled className='bg-white p-1.5 w-full rounded-sm border border-gray-500 text-gray-800 text-xs font-normal drop-shadow-md uppercase
            duration-150 ease-in-out hover:bg-gray-800 hover:text-white hover:border-white disabled:bg-gray-300 disabled:pointer-events-none'> simular </button>
          </div>
          <div className='flex-1 p-2 flex flex-col items-center justify-end'>
            <p className='text-[0.55em] text-gray-300'> <span className='uppercase font-semibold'>Metaheurística: </span> Nearest Neighbours</p>
            <p className='text-[0.6em] text-gray-300 px-1.5'>@dev - v0.1</p>
          </div>
        </div>
        <div data-attr="sandbox-container" className='row-span-1 col-span-6 grid grid-rows-6 gap-2'>
          <div data-attr="sandbox" className='col-span-1 row-span-5 rounded-md border border-gray-500 flex flex-col'>
            <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>sandbox</p>
            <div className='flex-1 w-full bg-gray-200 rounded-b-md'>
              <GraphSandbox></GraphSandbox>
            </div>
          </div>
          <div data-attr="output-container" className='col-span-1 row-span-1 rounded-md border border-gray-500'>
            <p className='w-full text-[0.6em] text-gray-300 font-semibold uppercase p-1 border-b border-gray-500'>Output</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
