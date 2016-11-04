#!/usr/bin/env groovy

node {
	stage 'Build'
	#Add tools to environment path
	#so sh calls don't need a path.
	#env.PATH = "${tool 'Maven 3'}/bin:${env.PATH}"

	#Pull the current branch's source from the 
	#source control system.
	checkout scm

	#Perform the build.
	#Build the engine...
	sh 'cargo build --manifest-path src/open-source/engine/modules/Cargo.toml'
	#Build any tools here.

	stage 'Test'
	#Similarly, test the engine.
	#Now test the tools if possible.
}