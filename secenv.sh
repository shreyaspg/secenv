#!/bin/bash
set -x
#get args passed to script
args=("$@")

#get number of args
NUM_ARGS=$#

#binary dir
BIN_DIR="./target/debug/"

#binary name
BIN_NAME="secenv"

#binary path
BIN_PATH=$BIN_DIR$BIN_NAME
#usage string
command_usage="Usage: ./secenv.sh [init | profile ]" 
subcommand_usage="Usage: ./secenv.sh  profile [ set | unset | list ]"
set_usage="Usage: ./secenv.sh  profile set <profile_name>"
unset_usage="Usage: ./secenv.sh  profile unset <profile_name>"

#check if NUM_ARGS is greater than 1
if [ $NUM_ARGS -le 1 ]; then
    echo $usage
    exit 1
fi

#check if first arg is init or profile
if [ ${args[0]} != "init" ] && [ ${args[0]} != "profile" ]; then
    echo $command_usage
    exit 1
fi

command=${args[0]}

#check if command is init
if [ ${command} == "init" ];then
    #run the eval command to call the binary
    eval $BIN_PATH $command
    exit 0
fi

#check if command is profile
if [ ${command} == "profile" ];then
    #check if subcommand is passed
    if [ -z ${args[1]} ]; then
        echo $subcommand_usage
        exit 1
    fi

    subcommand=${args[1]}

    #Check if subcommand is "set" or "unset" or "list"
    if [ ${subcommand} != "set" ] && [ ${subcommand} != "unset" ] && [ ${subcommand} != "list" ] ; then
        echo $subcommand_usage
        exit 1
    fi

    # check if subcommand is "set"
    if [ ${subcommand} == "set" ]; then
        if [ -z ${args[2]} ]; then
            echo $set_usage
            exit 1
        fi

        profile_name=${args[2]}
        #run the eval command to call the binary
        eval $BIN_PATH $command $subcommand $profile_name
        exit 0
    fi

    # check if subcommand is "unset"
    if [ ${subcommand} == "unset" ]; then
        if [ -z ${args[2]} ]; then
            echo $unset_usage
            exit 1
        fi

        profile_name=${args[2]}
        #run the eval command to call the binary
        eval $BIN_PATH $command $subcommand $profile_name
        exit 0
    fi
    #run the eval command to call the binary
    eval $BIN_PATH $command $subcommand

    #check if the command ran successfully
    if [ $? -ne 0 ]; then
        echo $subcommand_usage
        exit 1
    fi
fi

