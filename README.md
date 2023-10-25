# Project Overview

This a tauri application, built with react as the frontend framework.

The application is composed by a graph sandbox UI, where you can define and see the graphs you are creating, and a 'Branch and Bound' algorithm (rust backend) adapted to solve the 'travelling salesman problem'.

![Alt text](1.png?raw=true "Application")

## Available Commands

<strong>npm run tauri dev</strong> - Use this command to serve the application in development mode.

<strong>npm run tauri build</strong> - Use this command to build the application installer.

## Project Requirements

To build/develop the project you will need **Rust** and **Cargo** (Rust package manager), you will also need **node** and **npm**.

## Considerations

If you are to make something out of this project, i would advise you to add multi thread support, currently this application does not support it, so if you were to run the application on a bigger graph the ui will freeze while the backend solves **tsp**.
