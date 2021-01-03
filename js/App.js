import React, { Component } from 'react';
import { createMuiTheme, makeStyles } from "@material-ui/core/styles";
import {
    Box, Button, Divider, Grid, GridList, GridListTile,
    GridListTileBar, InputBase, Modal, Tab, Tabs, TextField, Typography
} from '@material-ui/core';
import PropTypes from 'prop-types';
import { ThemeProvider } from '@material-ui/styles';
import BugReportIcon from '@material-ui/icons/BugReport';
import NotesIcon from '@material-ui/icons/Notes';
import IconButton from '@material-ui/core/IconButton';
import DeleteIcon from '@material-ui/icons/Delete';
import MenuBookIcon from '@material-ui/icons/MenuBook';
import SettingsOutlinedIcon from '@material-ui/icons/SettingsOutlined';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import GetAppIcon from '@material-ui/icons/GetApp';
import PublishIcon from '@material-ui/icons/Publish';
import TreeView from '@material-ui/lab/TreeView';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';
import TreeItem from '@material-ui/lab/TreeItem';

import { useLocalStore, useObserver, observer } from "mobx-react";
import { makeObservable, observable, action, computed } from "mobx"

import wabt from "wabt";

import './App.css';

const regeneratorRuntime = require("regenerator-runtime");


class ObservableStateStore {
    imageInputFiles = [
        {
            name: 'img_1',
            src: "https://www.gettyimages.com/gi-resources/images/500px/983794168.jpg",
            width: 1,
            height: 1,
            dataUint8: [],
            // cols: 2,
        },
        {
            name: 'img_2',
            src: "https://helpx.adobe.com/content/dam/help/en/stock/how-to/visual-reverse-image-search/jcr_content/main-pars/image/visual-reverse-image-search-v2_intro.jpg",
            width: 1,
            height: 1,
            dataUint8: [],
            // cols: 2,
        },
        {
            name: 'img_3',
            src: "https://photojournal.jpl.nasa.gov/jpeg/PIA23689.jpg",
            width: 1,
            height: 1,
            dataUint8: [],
            // cols: 2,
        },
    ];
    imageOutputFiles = [
        {
            name: 'img_10',
            src: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTgaNi3D4eNMnyGH56_1gD-0485xuI07N6ztw&usqp=CAU",
            width: 1,
            height: 1,
            dataUint8: [],
        },
        {
            name: 'img_11',
            src: "https://image.shutterstock.com/image-photo/large-beautiful-drops-transparent-rain-260nw-668593321.jpg",
            width: 1,
            height: 1,
            dataUint8: [],
        }
    ];
    code = "";
    consoleOutput = "";
    consoleWasm = "";

    constructor() {
        makeObservable(this, {
            imageInputFiles: observable,
            imageOutputFiles: observable,
            code: observable,
            consoleOutput: observable,
            consoleWasm: observable,
            addInputImage: action,
            renameInputImage: computed,
            deleteInputImage: action,
            addOutputImage: action,
            changeCode: action,
            printConsoleOutput: action,
            printConsoleWasm: action,
        });
    }

    addInputImage(image) {
        this.imageInputFiles.push(image);
        console.log(image)
        console.log(this.imageInputFiles)
    }

    // get renameInputImage(oldName, newName) {
    get renameInputImage() {
        var success = true;
        for (var v of this.imageInputFiles) {
            if (v.name == newName) {
                success = false;
            }
        }
        return success
    }

    get deleteInputImage() {

    }

    addOutputImage(image) {
        this.imageOutputFiles.push(image);
        console.log(image)
        console.log(this.imageOutputFiles)
    }

    changeCode(src) {
        this.code = src;
        console.log(this.code)
    }

    printConsoleOutput(out) {
        this.consoleOutput = out;
        console.log(this.consoleOutput)
    }

    printConsoleWasm(wasm) {
        this.consoleWasm = wasm;
        console.log(this.consoleWasm)
    }
}

const observableStateStore = new ObservableStateStore();













const theme = createMuiTheme({
    palette: {
        primary: {
            main: "#fbc02d",
        },
    }

});

const useStyles = makeStyles(() => ({
    root: {
        height: "100vh",
        flexGrow: 1,
    },
    menuButton: {
        marginRight: theme.spacing(2),
    },
    title: {
        flexGrow: 1,
        textAlign: "center"
    },
    leftSider: {
        flexGrow: 1,
        display: 'flex',
        height: "100vh",
        borderRight: "1px solid",
        borderColor: theme.palette.secondary.main,
    },
    leftSiderTabs: {
        width: 48,
        backgroundColor: theme.palette.primary.main,
    },
    indicator: {
        left: 0,
        color: theme.palette.secondary.main,
        width: 2
    },
    leftSiderTab: {
        minWidth: 48,
        width: 48,
        color: "white",
    },
    leftSiderTabPanels: {
        padding: theme.spacing(1),
        flexGrow: 1,
        width: 'calc(23vw - 48px)'
    },
    codeBlock: {
        height: "100%",
        borderRight: "1px solid",
        borderColor: theme.palette.secondary.main,
    },
    bar: {
        width: '100%',
    },
    codeText: {
        height: 'fit-content',
        width: "100%",
        padding: theme.spacing(.5)

    },
    codeConsole: {
        width: 'fit-content',
    },
    codeConsoleTab: {
        minWidth: 48,
        color: theme.palette.text.secondary,
    },
    imageBlock: {
        borderRight: "1px solid",
        borderColor: theme.palette.secondary.main,
    },
    inputIcon: {
        display: 'none',
    },
    imageBlockSplit: {
        maxHeight: "50vh",
    },
    imageInputPanel: {
        display: 'flex',
        flexWrap: 'wrap',
        overflow: 'scroll',
        height: 'calc(50vh - 48px)',
        padding: theme.spacing(.5),
    },
    gridList: {
        flexWrap: 'nowrap',
        transform: 'translateZ(0)',
    },
    imageTitleBar: {
        background:
            // 'linear-gradient(to top, rgba(0,0,0,0.7) 0%, rgba(0,0,0,0.3) 60%, rgba(0,0,0,0) 90%)',
            'linear-gradient(to top, rgba(255,255,255,0.9) 0%, rgba(255,255,255,0.5) 80%, rgba(255,255,255,0) 100%)',
    },
    imageRenamePaper: {
        position: "absolute",
        backgroundColor: theme.palette.background.paper,
        borderRadius: "5",
        boxShadow: theme.shadows[5],
        padding: theme.spacing(3, 5, 2),
    },
    imageRenameButtons: {
        "& > *": {
            margin: theme.spacing(1)
        }
    },
    imageRenameContainer: {
        display: "grid",
        gridGap: theme.spacing(1)
    },
}));

export default function App() {
    const classes = useStyles();
    return (
        <ThemeProvider theme={theme}>
            <Grid container className={classes.root}>
                <Grid item md={3}>
                    <LeftSider></LeftSider>
                </Grid>
                <Grid item md={4}>
                    <CodeBlock></CodeBlock>
                </Grid>
                <Grid item md={5}>
                    <ImageBlock></ImageBlock>
                </Grid>
            </Grid>
        </ThemeProvider>
    );
}

function TabPanel(props) {
    const { children, value, index } = props;

    return (
        <div
            role="tabpanel"
            id={`vertical-tabpanel-${index}`}
        // {...other}
        >
            {value === index && (
                <Box>{children}</Box>
            )}
        </div>
    );
}

TabPanel.propTypes = {
    children: PropTypes.node,
    index: PropTypes.any.isRequired,
    value: PropTypes.any.isRequired,
};

function a11yProps(index) {
    return {
        id: `vertical-tab-${index}`,
        'aria-controls': `vertical-tabpanel-${index}`,
    };
}

function LeftSider() {
    const classes = useStyles();
    const [value, setValue] = React.useState(0);

    const handleChange = (event, newValue) => {
        setValue(newValue);

    };

    return (
        <div className={classes.leftSider}>
            <Tabs
                orientation="vertical"
                value={value}
                onChange={handleChange}
                className={classes.leftSiderTabs}
                classes={{ indicator: classes.indicator }}
            >
                <Tab icon={<MenuBookIcon />} {...a11yProps(0)} className={classes.leftSiderTab} />
                <Tab icon={<SettingsOutlinedIcon />} {...a11yProps(1)} className={classes.leftSiderTab} />
            </Tabs>
            <TabPanel value={value} index={0}  >
                <LeftSiderDocument></LeftSiderDocument>
            </TabPanel>
            <TabPanel value={value} index={1} className={classes.leftSiderTabPanels} >
                Item Two
            </TabPanel>
        </div>
    );
}

function LeftSiderDocument() {
    const classes = useStyles();
    return (
        <TreeView
            className={classes.leftSiderTabPanels}
            defaultCollapseIcon={<ExpandMoreIcon />}
            defaultExpandIcon={<ChevronRightIcon />}
        >
            <TreeItem nodeId="blank_image_main" label="blank_image([n1], [n2]) -> [n3]">
                <TreeItem nodeId="blank_image_inp" label="input">
                    <TreeItem nodeId="blank_image_inp_1" label="[n1] width of an image" />
                    <TreeItem nodeId="blank_image_inp_2" label="[n2] height of an image" />
                </TreeItem>
                <TreeItem nodeId="blank_image_rtn" label="return">
                    <TreeItem nodeId="blank_image_rtn_1" label="[n3] image_id" />
                </TreeItem>
                <TreeItem nodeId="blank_image_exp" label="explanation">
                    <TreeItem nodeId="blank_image_exp_1" label="create a white image with given width and given height" />
                </TreeItem>
                <TreeItem nodeId="blank_image_usg" label="usage">
                    <TreeItem nodeId="blank_image_usg_1" label="img_1 = blank_image(50,10);" />
                </TreeItem>
            </TreeItem>
            <TreeItem nodeId="darken_main" label="darken([img1], [n2]) -> [n3]">
                <TreeItem nodeId="darken_inp" label="input">
                    <TreeItem nodeId="darken_inp_1" label="[img1] an image id" />
                    <TreeItem nodeId="darken_inp_2" label="[n2] a brightness value to reduce" />
                </TreeItem>
                <TreeItem nodeId="darken_rtn" label="return">
                    <TreeItem nodeId="darken_rtn_1" label="[n3] image_id" />
                </TreeItem>
                <TreeItem nodeId="darken_exp" label="explanation">
                    <TreeItem nodeId="darken_exp_1" label="darken a target image with given a given value by reducing red, green and blue channel all
                      by the given value. The given value should be within 0-255, a channel whose value surpasses the limit will
                      be adjusted to boudary 0 or 255 accordingly" />
                </TreeItem>
                <TreeItem nodeId="darken_usg" label="usage">
                    <TreeItem nodeId="darken_usg_1" label="img_1 = darken(grassland,10);" />
                </TreeItem>
            </TreeItem>
        </TreeView>
    );
}

function CodeBlock() {
    const classes = useStyles();
    return (
        <Grid container direction="column" className={classes.codeBlock}>
            <CodeBar></CodeBar>
            <Divider />
            <CodeText></CodeText>
            <Divider />
            <CodeConsole></CodeConsole>
        </Grid>
    );
}

function CodeBar() {
    const classes = useStyles();
    return (
        <Grid container className={classes.bar}>
            <Grid item md container justify="flex-start" color="theme.palette.text.secondary">
                <input accept=".txt" className={classes.inputIcon} id="code_upload" type="file" />
                <label htmlFor="code_upload" >
                    <IconButton id="code_upload_icon">
                        <PublishIcon />
                    </IconButton>
                </label>
                <IconButton>
                    <GetAppIcon />
                </IconButton>
                <Divider orientation="vertical" />
            </Grid>
            <Grid item md container justify="flex-end" color="theme.palette.text.secondary">
                <Divider orientation="vertical" />
                <IconButton id="run">
                    <PlayCircleFilledIcon />
                </IconButton>
            </Grid>
        </Grid>

    );
}

const CodeText = observer(() => {
    const classes = useStyles();
    const handleChange = (event) => {
        observableStateStore.changeCode(event.target.value)
    };
    return (
        <Grid container justify="flex-end" >
            <form className={classes.codeText} noValidate autoComplete="off">
                <InputBase
                    id="code_text"
                    multiline
                    rows={20}
                    placeholder="Enter your code here"
                    className={classes.codeText}
                    onChange={handleChange}
                />
            </form>
        </Grid>
    );
})

function CodeConsole() {
    const classes = useStyles();
    const [value, setValue] = React.useState(0);

    const handleChange = (event, newValue) => {
        setValue(newValue);
    };
    return (
        <Grid container direction="column">
            <Tabs value={value} onChange={handleChange} aria-label="code_console">
                <Tab icon={<NotesIcon />} {...a11yProps(0)} className={classes.codeConsoleTab} />
                <Tab icon={<BugReportIcon />} {...a11yProps(1)} className={classes.codeConsoleTab} />
            </Tabs>
            <Divider />
            <TabPanel value={value} index={0}>
                <CodeConsoleWasm />
            </TabPanel>
            <TabPanel value={value} index={1} >
                <CodeConsoleMessage />
            </TabPanel>
        </Grid >
    );
}

function CodeConsoleMessage() {
    const classes = useStyles();
    return (
        <form className={classes.codeText} noValidate autoComplete="off">
            <InputBase
                id="console"
                multiline
                rowsMax={11}
                readOnly
                value="console output"
                className={classes.codeText}
            />
        </form>
    );
}

const CodeConsoleWasm = observer(() => {
    const classes = useStyles();
    return (
        <form className={classes.codeText} noValidate autoComplete="off">
            <InputBase
                id="wasm_output"
                multiline
                rowsMax={11}
                readOnly
                value={observableStateStore.consoleWasm}
                placeholder="wasm_output"
                className={classes.codeText}
            />
        </form>
    );
})



function ImageBlock() {
    const classes = useStyles();
    return (
        <Grid container direction={"column"} className={classes.imageBlock}>
            <Grid item container direction="column">
                <ImageInputBar></ImageInputBar>
                <Divider />
                <ImageInputPanel></ImageInputPanel>
            </Grid >
            <Divider />
            <Grid item container direction="column" >
                <ImageOutputBar></ImageOutputBar>
                <Divider />
                <ImageOutputPanel></ImageOutputPanel>
            </Grid >
        </Grid>
    );
}

function ImageInputBar() {
    const classes = useStyles();
    const handleUpload = (event) => {
        [].forEach.call(event.target.files, function read_file(file) {
            if (/\.(jpe?g|png|gif)$/i.test(file.name)) {
                var image = {
                    name: file.name.split(".")[0],
                    src: "",
                    width: 0,
                    height: 0,
                    dataUint8: [],
                };
                var reader = new FileReader();
                reader.onload = function () {
                    image.src = reader.result;
                    var img = new Image();
                    img.onload = function () {
                        image.height = img.height;
                        image.width = img.width;
                        var imgContext = document.createElement("canvas").getContext("2d");
                        imgContext.drawImage(img, 0, 0);
                        var imageData = imgContext.getImageData(0, 0, img.width, img.height).data;
                        image.dataUint8 = new Uint8Array(imageData.buffer);
                        observableStateStore.addInputImage(image);
                    }
                    img.src = image.src;
                };
                reader.readAsDataURL(file);
            }
        });
    };
    return (
        <Grid container className={classes.bar}>
            <input accept="image/*"
                className={classes.inputIcon}
                id="image_upload" type="file" onChange={handleUpload} />
            <label htmlFor="image_upload" >
                <IconButton component="span" id="image_upload_icon" >
                    <PublishIcon />
                </IconButton>
            </label>
            {/* <IconButton>
                <DeleteIcon />
            </IconButton> */}
        </Grid>
    );
}

function ImageInputTile({ tile }) {
    // const classes = useStyles();

    // const [value, setValue] = React.useState(tile.name);
    // const handleChange = (event) => {
    //     setValue(event.target.value);
    // };


    return (
        <GridListTile key={tile.name} cols={2} >
            <img src={tile.src} alt={tile.name} />
            <GridListTileBar
                // className={classes.imageTitleBar}
                title={
                    <form noValidate autoComplete="off">
                        {/* <InputBase value={tile.name} id={tile.name} onChange={handleChange}> */}
                        <InputBase value={tile.name} id={tile.name} >
                        </InputBase>
                    </form>
                }
                actionIcon={
                    <IconButton>
                        <DeleteIcon></DeleteIcon>
                    </IconButton>
                }
            />
        </GridListTile >
    )
}

const ImageInputPanel = observer(() => {
    const classes = useStyles();
    return (
        <GridList cellHeight={150}
            className={classes.imageInputPanel}
            cols={8}>
            {observableStateStore.imageInputFiles.map(
                tile => ImageInputTile({ tile })
            )}
        </GridList>
    );
})

function ImageOutputBar() {
    const classes = useStyles();
    return (
        <Grid container className={classes.bar}>
            <IconButton id="upload_icon">
                <GetAppIcon />
            </IconButton>
        </Grid>
    );
}

let ImageOutputTile = ({ tile }) => {
    const classes = useStyles();

    const [value, setValue] = React.useState(tile.name);
    const handleChange = (event) => {
        setValue(event.target.value);
    };

    return (
        <GridListTile key={tile.name} cols={2} >
            <img src={tile.src} alt={tile.name} />
            <GridListTileBar
                className={classes.imageTitleBar}
                title={
                    <form noValidate autoComplete="off">
                        <InputBase value={tile.name} id={tile.name} onChange={handleChange}>
                        </InputBase>
                    </form>
                }
                actionIcon={
                    <IconButton>
                        <GetAppIcon></GetAppIcon>
                    </IconButton>
                }
            />
        </GridListTile >
    )
};

function ImageOutputPanel() {
    const classes = useStyles();
    return (
        <GridList cellHeight={150} className={classes.imageInputPanel} cols={8}>
            {observableStateStore.imageOutputFiles.map((tile) => ImageOutputTile({ tile }))}
        </GridList>
    );
}

async function main() {
    let compiler = await import("../pkg/compiler.js");

    function processImageInput() {
        var names = [];
        for (var v of observableStateStore.imageInputFiles) {
            names.push(v.name);
            compiler.add_image_bindgen(v.name, v.width, v.height, v.dataUint8);
        }
        console.log(names);
        return names;
    }

    document.getElementById('run').onclick = async function () {
        var image_names = processImageInput();
        var output_wasm_buffer = compiler.code_to_wasm(observableStateStore.code, image_names);
        print_wat(output_wasm_buffer);

        var importObject = {
            env: {
                darken: function (img_id, value) {
                    compiler.darken(img_id, value);
                },
                blank_image: function (width, height) {
                    compiler.blank_image(width, height);
                },
            }
        };
        let { _, instance } = await WebAssembly.instantiate(output_wasm_buffer, importObject);//?
        instance.exports.main();


        async function print_wat(buffer) {
            var w = await wabt()
            var module = w.readWasm(buffer, { readDebugNames: true });
            module.generateNames();
            module.applyNames();
            var wat = module.toText({
                foldExprs: true,
                inlineExport: false
            });
            observableStateStore.printConsoleWasm(wat);
        }

        // show_result_images(compiler.export_bindgen());
    }



    //     function image_to_src(data, width, height) {
    //         var canvas = document.createElement("canvas");
    //         canvas.width = width;
    //         canvas.height = height;
    //         var context = canvas.getContext("2d");
    //         var imageData = context.createImageData(width, height);
    //         imageData.data.set(data);
    //         context.putImageData(imageData, 0, 0);
    //         return canvas.toDataURL()
    //     }
}

main();





