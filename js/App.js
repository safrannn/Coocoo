import React, { Component } from 'react';
import { createMuiTheme, makeStyles } from "@material-ui/core/styles";
import {
    AppBar, Box, Button, ButtonGroup, Divider, Grid, GridList, GridListTile,
    GridListTileBar, IconButton, InputBase, Snackbar, Tab, Tabs, TextField,
    Toolbar, Typography
} from '@material-ui/core';
import { TreeItem, TreeView } from '@material-ui/lab';
import MuiAlert from '@material-ui/lab/Alert';
import PropTypes from 'prop-types';
import { ThemeProvider } from '@material-ui/styles';
import BugReportIcon from '@material-ui/icons/BugReport';
import NotesIcon from '@material-ui/icons/Notes';
import DeleteIcon from '@material-ui/icons/Delete';
import MenuBookIcon from '@material-ui/icons/MenuBook';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import GetAppIcon from '@material-ui/icons/GetApp';
import PublishIcon from '@material-ui/icons/Publish';
import RemoveCircleOutlineIcon from '@material-ui/icons/RemoveCircleOutline';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';

import { useLocalStore, useObserver, observer } from "mobx-react";
import { makeObservable, observable, action, computed } from "mobx"

import wabt from "wabt";

import './App.css';
import { KeyboardReturn } from '@material-ui/icons';

const regeneratorRuntime = require("regenerator-runtime");


class ObservableStateStore {
    imageInputFiles = new Map([]);
    imageOutputFiles = new Map([]);
    code = "";
    consoleMessage = "";
    consoleWasm = "";

    constructor() {
        makeObservable(this, {
            imageInputFiles: observable,
            imageOutputFiles: observable,
            code: observable,
            consoleMessage: observable,
            consoleWasm: observable,
            addInputImage: action,
            renameInputImage: action,
            deleteInputImage: action,
            addOutputImage: action,
            renameOutputImage: action,
            clearOutputImage: action,
            changeCode: action,
            clearCode: action,
            printConsoleMessage: action,
            addConsoleMessage: action,
            clearConsoleMessage: action,
            printConsoleWasm: action,
        });
    }

    addInputImage(name, image) {
        this.imageInputFiles.set(name, image);
    }

    renameInputImage(oldName, newName) {
        if (this.imageInputFiles.has(newName)) {
            return false;
        } else {
            this.imageInputFiles.set(newName, this.imageInputFiles.get(oldName));
            this.imageInputFiles.delete(oldName);
            return true
        }
    }

    deleteInputImage(name) {
        this.imageInputFiles.delete(name);
    }

    addOutputImage(category, image_name, image_data) {
        if (!this.imageOutputFiles.get(category)) {
            this.imageOutputFiles.set(category, {});
        }
        this.imageOutputFiles.get(category).image_name = image_data;
    }

    renameOutputImage(category, oldName, newName) {
        if (!this.imageOutputFiles.get(category).hasOwnProperty(newName)) {
            return false
        } else {
            var image_data = this.imageOutputFiles.get(category).oldName;
            delete this.imageOutputFiles.get(category).oldName;
            this.imageOutputFiles.get(category).newName = image_data;
            return true
        }
    }

    clearOutputImage() {
        this.imageOutputFiles.clear();
    }

    changeCode(src) {
        this.code = src;
    }

    clearCode() {
        this.code = "";
    }

    printConsoleMessage(out) {
        this.consoleMessage = out;
    }
    addConsoleMessage(errorMessage) {
        this.consoleMessage += errorMessage + "\n"
    }
    clearConsoleMessage() {
        this.consoleMessage = "";
    }
    printConsoleWasm(wasm) {
        this.consoleWasm = wasm;
    }
}

const observableStateStore = new ObservableStateStore();


window.S = observableStateStore;


const theme = createMuiTheme({
    palette: {
        primary: {
            main: "#fbc02d",
        },
        secondary: {
            main: "#3f51b5",
        },
        warning: {
            main: "#d81b60",
        },
        divider: "#bdbdbd",
    }
});

const useStyles = makeStyles((theme) => ({
    root: {
        height: "100vh",
        flexGrow: 1,
    },
    customizeToolbar: {
        minHeight: 45
    },
    menuButton: {
        marginRight: theme.spacing(2),
    },
    title: {
        flexGrow: 1,
        textAlign: "center",
        color: "white",
    },
    leftSider: {
        flexGrow: 1,
        display: 'flex',
        height: "calc(100vh - 45px)",
        borderRight: `1px solid ${theme.palette.divider}`
    },
    leftSiderTabs: {
        borderRight: `1px solid ${theme.palette.divider}`,
        indicatorColor: "#d81b60",
    },
    indicator: {
        left: 0,
        color: theme.palette.primary.main,
        width: 2
    },
    leftSiderTab: {
        minWidth: 48,
        width: 48,
        color: "#9e9e9e",
    },
    leftSiderTabPanels: {
        flexGrow: 1,
    },
    codeBlock: {
        // height: "calc(100vh - 45px)",
        // borderRight: `1px solid ${theme.palette.divider}`,
    },
    codeBar: {
        width: '100%',
        borderBottom: `1px solid ${theme.palette.divider}`,
    },
    codeText: {
        width: "100%",
        padding: theme.spacing(1),
        height: "calc(3*(100vh - 45px - 48px - 48px)/5 - 1)",
    },
    codeConsole: {
        // width: 'fit-content',
        width: "100%",
        flexGrow: 1,
        display: "flex",
        height: "calc(2*(100vh - 45px - 48px - 48px)/5 - 1)",

    },
    codeConsoleTab: {
        minWidth: 48,
        color: theme.palette.text.secondary,
    },
    imageBlock: {
        height: "calc(100vh - 45px)",
        borderLeft: `1px solid ${theme.palette.divider}`
    },
    inputIcon: {
        display: 'none',
    },
    imageBlockSplit: {
        // maxHeight: "calc((100vh - 45px)/2)",
    },
    imageInputTab: {
        display: 'flex',
        flexWrap: 'wrap',
        overflow: 'scroll',
        // height: 'calc((100vh - 45px)/2 - 48px)',
        padding: theme.spacing(.5),
    },
    gridList: {
        flexWrap: 'nowrap',
        transform: 'translateZ(0)',
    },
    imageTitleBar: {
        background:
            'linear-gradient(to top, rgba(0,0,0,0.9) 0%, rgba(0,0,0,0.5) 40%, rgba(0,0,0,0) 70%)',
        color: "white",
    },
    imageOutputFileList: {
        // flexGrow: 1,
        backgroundColor: theme.palette.background.paper,
        // display: 'flex',
        width: "100px",
        height: 'calc((100vh - 45px- 48px)/2)',
    },
    imageOutputFileListTab: {
        borderRight: `1px solid ${theme.palette.divider}`,
    },
}));

export default function App() {
    const classes = useStyles();
    return (
        <ThemeProvider theme={theme}>
            <Grid container className={classes.root}>
                <Grid item xs={12}>
                    <TitleBar />
                </Grid>
                <Grid container >
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
            </Grid >
            <Divider />
        </ThemeProvider >
    );
}

function TitleBar() {
    const classes = useStyles();
    return (
        <div>
            <AppBar position="static" elevation={0} color="primary" >
                <Toolbar className={classes.customizeToolbar}>
                    <Typography align="center" className={classes.title}>
                        Coocoo Material Generator
                    </Typography>
                </Toolbar>
            </AppBar>
        </div>
    );
}

function TabPanel(props) {
    const { children, value, index, prefix, ...other } = props;

    return (
        <div
            role="tabpanel"
            hidden={value !== index}
            id={`${prefix}-${index}`}
            aria-labelledby={`${prefix}-${index}`}
            {...other}
        >
            {value === index && (
                <Box p={1.5}>
                    {children}
                </Box>
            )}
        </div>
    );
}

TabPanel.propTypes = {
    children: PropTypes.node,
    index: PropTypes.any.isRequired,
    value: PropTypes.any.isRequired,
    prefix: PropTypes.any.isRequired
};

function a11yProps(prefix, index) {
    return {
        id: `${prefix}-${index}`,
        "aria-controls": `${prefix}-${index}`
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
            >
                <Tab icon={<MenuBookIcon />} {...a11yProps("leftSider-tabs", 0)} className={classes.leftSiderTab} />
            </Tabs>
            <TabPanel value={value} index={0} prefix="leftSider-tabs">
                <LeftSiderDocument></LeftSiderDocument>
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
            <TreeItem nodeId="darken_main" label="darken([img1], [n1]) -> [n2]">
                <TreeItem nodeId="darken_inp" label="input">
                    <TreeItem nodeId="darken_inp_1" label="[img1] an image id" />
                    <TreeItem nodeId="darken_inp_2" label="[n1] a brightness value to reduce" />
                </TreeItem>
                <TreeItem nodeId="darken_rtn" label="return">
                    <TreeItem nodeId="darken_rtn_1" label="[n2] image_id" />
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
            <TreeItem nodeId="grayscale_main" label="grayscale([img1]) -> [n1]">
                <TreeItem nodeId="grayscale_inp" label="input">
                    <TreeItem nodeId="grayscale_inp_1" label="[img1] an image id" />
                </TreeItem>
                <TreeItem nodeId="grayscale_rtn" label="return">
                    <TreeItem nodeId="grayscale_rtn_1" label="[n1] image_id" />
                </TreeItem>
                <TreeItem nodeId="grayscale_exp" label="explanation">
                    <TreeItem nodeId="grayscale_exp_1" label="generate grayscale of an image" />
                </TreeItem>
                <TreeItem nodeId="grayscalen_usg" label="usage">
                    <TreeItem nodeId="grayscale_usg_1" label="img_1 = grayscale(sky);" />
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
            <CodeText></CodeText>
            <Divider />
            <CodeConsole></CodeConsole>
        </Grid>
    );
}

function CodeBar() {
    const classes = useStyles();
    const handleClearCode = (event) => {
        observableStateStore.clearCode();
    };
    return (
        <Grid container className={classes.codeBar}>
            <Grid item md container justify="flex-start" color="theme.palette.text.secondary">
                {/* <input accept=".txt" className={classes.inputIcon} id="code_upload" type="file" />
                <label htmlFor="code_upload" >
                    <IconButton id="code_upload_icon">
                        <PublishIcon />
                    </IconButton>
                </label>
                <IconButton>
                    <GetAppIcon />
                </IconButton>
                <Divider orientation="vertical" /> */}
                <IconButton onClick={handleClearCode}>
                    <RemoveCircleOutlineIcon />
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
                    value={observableStateStore.code}
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
        <Grid>
            <Tabs value={value} onChange={handleChange} aria-label="code_console">
                <Tab icon={<NotesIcon />} {...a11yProps("codeConsole-tabs", 0)} />
                <Tab icon={<BugReportIcon />} {...a11yProps("codeConsole-tabs", 1)} />
            </Tabs>
            <Divider />
            <TabPanel value={value} index={0} prefix="codeConsole-tabs">
                <CodeConsoleMessage />
            </TabPanel>
            <TabPanel value={value} index={1} prefix="codeConsole-tabs">
                <CodeConsoleWasm />
            </TabPanel>
        </Grid >
    );
}

const CodeConsoleMessage = observer(() => {
    const classes = useStyles();
    return (
        <form noValidate autoComplete="off">
            <InputBase
                id="console_message"
                multiline
                rows={7}
                rowsMax={7}
                readOnly
                value={observableStateStore.consoleMessage}
                placeholder="console"
                className={classes.codeConsole}
            />
        </form>
    );
})

var console_log = console.log;
console.log = function (message) {
    observableStateStore.addConsoleMessage(message);
    console_log.apply(console, arguments);
};

const CodeConsoleWasm = observer(() => {
    const classes = useStyles();
    return (
        <form noValidate autoComplete="off">
            <InputBase
                id="console_wasm"
                multiline
                rows={7}
                rowsMax={7}
                readOnly
                value={observableStateStore.consoleWasm}
                placeholder="wasm text"
                className={classes.codeConsole}
            />
        </form>
    );
})


function ImageBlock() {
    const classes = useStyles();
    const [value, setValue] = React.useState(0);

    const handleChange = (event, newValue) => {
        setValue(newValue);
    };
    return (
        // <div className={classes.imageBlock}>
        //     <ImageUpload />
        //     <ImageInputTab />
        // </div>
        <Grid container className={classes.imageBlock}>
            <Grid item md container justify="flex-start">
                <Grid container direction="column">
                    <Tabs variant="fullWidth" value={value} onChange={handleChange} aria-label="image block">
                        <Tab label="Input" {...a11yProps("imageBlock-tabs", 0)} className={classes.codeConsoleTab} />
                        <Tab label="Output" {...a11yProps("imageBlock-tabs", 1)} className={classes.codeConsoleTab} />
                    </Tabs>
                    <Divider />
                    <TabPanel value={value} index={0} prefix="imageBlock-tabs">
                        <Grid container>
                            <Grid item align="center" xs={12}>
                                <ImageUpload />
                            </Grid>
                            <Grid item xs={12}>
                                <ImageInputTab />
                            </Grid>
                        </Grid>
                    </TabPanel>
                    <TabPanel value={value} index={1} prefix="imageBlock-tabs">
                        <ImageOutputTab />
                    </TabPanel>
                </Grid >
            </Grid>
        </Grid >
    );
}

function Alert(props) {
    return <MuiAlert elevation={6} variant="filled" {...props} />;
}

function ImageUpload() {
    const classes = useStyles();
    const [open, setOpen] = React.useState(false);
    const handleClose = (event, reason) => {
        if (reason === 'clickaway') {
            return;
        }
        setOpen(false);
    };
    const handleUpload = (event) => {
        [].forEach.call(event.target.files, function read_file(file) {
            if (/.*?[^0-9].*\.(jpe?g|png|gif)$/i.test(file.name)) {
                let imageName = file.name.split(".")[0];
                let image = {
                    src: "",
                    width: 0,
                    height: 0,
                    pixels: [],
                };
                let reader = new FileReader();
                reader.onload = function () {
                    image.src = reader.result;
                    let img = new Image();
                    img.onload = function () {
                        image.height = img.height;
                        image.width = img.width;
                        let imgContext = document.createElement("canvas").getContext("2d");
                        imgContext.drawImage(img, 0, 0);
                        let imageData = imgContext.getImageData(0, 0, img.width, img.height).data;
                        image.pixels = new Uint8Array(imageData.buffer);
                        observableStateStore.addInputImage(imageName, image);
                    }
                    img.src = image.src;
                };
                reader.readAsDataURL(file);
            } else {
                setOpen(true);
            }
        });
    };
    return (
        <div>
            <input accept="image/*" multiple
                className={classes.inputIcon}
                id="image_upload" type="file" onChange={handleUpload} />
            <label htmlFor="image_upload" >
                <IconButton component="span" id="image_upload_icon" >
                    <PublishIcon />
                </IconButton>
            </label>
            <Snackbar open={open} autoHideDuration={6000} onClose={handleClose}>
                <Alert onClose={handleClose} severity="warning">
                    Please upload an image with non numeric file name.
            </Alert>
            </Snackbar>
        </div>
    );
}

const ImageInputTab = observer(() => {
    const classes = useStyles();
    const items = [];
    observableStateStore.imageInputFiles.forEach(
        (tile, tileName) => {
            let name = tileName;
            let handleRename = (event) => {
                if (observableStateStore.renameInputImage(tileName, event.target.value)) {
                    name = event.target.value;
                }
            };
            let handleDelete = () => {
                observableStateStore.deleteInputImage(name);
            }

            items.push(
                <GridListTile key={tileName} cols={2} >
                    <img src={tile.src} alt={tileName} height="100%" />
                    <GridListTileBar
                        className={classes.imageTitleBar}
                        title={
                            <form noValidate autoComplete="off">
                                <InputBase value={name} id={name} margin="dense" autoFocus onChange={handleRename} >
                                </InputBase>
                            </form>
                        }
                        actionIcon={
                            <IconButton onClick={handleDelete}>
                                <DeleteIcon></DeleteIcon>
                            </IconButton>
                        }
                    />
                </GridListTile >
            )
        }
    )

    return (
        <GridList cellHeight={150} className={classes.imageInputPanel} cols={8}>
            {items}
        </GridList>
    );
})

const ImageOutputTab = observer(() => {
    return (
        <div>
            <ImageOutputFileList />
            {/* <ImageOutputRender /> */}
        </div>
    );
})

const ImageOutputFileList = observer(() => {
    const classes = useStyles();
    const [value, setValue] = React.useState(0);

    const handleChange = (event, newValue) => {
        setValue(newValue);
    };

    const tab = [];
    const tabPanel = [];

    let i = 0;
    for (var [category, info] of observableStateStore.imageOutputFiles) {
        tab.push(
            <Tab label={value} {...a11yProps("imageOutputFileList-tabs", 0)} />
        )
        tabPanel.push(
            <TabPanel value={value} index={i} prefix="imageOutputFileList-tabs" >
                <ImageOutputImageFileDisplay category={category} />
            </TabPanel>
        )
        i += 1;
    }
    // console.log(tab)
    // console.log(tabPanel)

    return (
        <div className={classes.imageOutputFileList}>
            <Tabs
                orientation="vertical"
                variant="scrollable"
                value={value}
                onChange={handleChange}
                className={classes.imageOutputFileListTab}
            >
                {/* <Tab label="Item One" {...a11yProps(0)} /> */}
                {tab}
            </Tabs>
            {/* <TabPanel value={value} index={0}>
                Item One
             </TabPanel> */}
            {tabPanel}
        </div>
    );
})

const ImageOutputImageFileDisplay = observer(({ category }) => {
    const classes = useStyles();

    let files = observableStateStore.imageOutputFiles.get(category);
    if (files == undefined) {
        console.log(category, " doesn't exist.")
        return
    }

    const items = [];
    for (const [tileName, tile] of Object.entries(files)) {
        let name = tileName;
        let handleRename = (event) => {
            if (observableStateStore.renameOutputImage(tileName, event.target.value)) {
                name = event.target.value;
            }
        };
        items.push(
            <GridListTile key={tileName} cols={2} >
                <img src={tile.src} alt={tileName} height="100%" />
                <GridListTileBar
                    className={classes.imageTitleBar}
                    title={
                        <form noValidate autoComplete="off">
                            <InputBase value={name} id={name} margin="dense" autoFocus onChange={handleRename} >
                            </InputBase>
                        </form>
                    }
                    actionIcon={
                        <Button href={tile.src} download={tileName}>
                            <GetAppIcon>
                            </GetAppIcon>
                        </Button>
                    }
                />
            </GridListTile >
        )
    }
    return (
        <GridList cellHeight={150} className={classes.imageInputTab} cols={8}>
            {items}
        </GridList>
    );
})

const ImageOutputRender = observer(() => {
    return
})


async function main() {
    let compiler = await import("../pkg/compiler.js");

    function processImageInput() {
        let names = [...observableStateStore.imageInputFiles.keys()];
        for (const [name, data] of observableStateStore.imageInputFiles) {
            compiler.library_add_image(name, data.width, data.height, data.pixels);
        }
        return names;
    }

    document.getElementById('run').onclick = async function () {
        compiler.library_reset();
        observableStateStore.clearConsoleMessage();

        let image_names = processImageInput();
        let output = compiler.code_to_wasm(observableStateStore.code, image_names);
        let output_wasm_buffer = new Uint8Array(output[0]);
        let output_image_info = output[1]; // image_name, image_id
        let output_material_info = output[2]; // position in mem, [material_name, channel_name]
        // console.log(output)
        // console.log("output_material_info", output_material_info)



        let has_error = observableStateStore.consoleMessage.length == 0 ? false : true
        if (!has_error) {
            observableStateStore.addConsoleMessage("\n✔ Compile finished.");
        }
        print_wat(output_wasm_buffer);

        let importObject = {
            env: {
                logger: function (arg) {
                    // console.log(arg);
                },
                darken: function (img_id, value) {
                    return compiler.darken(img_id, value)
                },
                blank_image: function (width, height) {
                    return compiler.blank_image(width, height)
                },
                grayscale: function (img_id) {
                    return compiler.grayscale(img_id)
                },
            }
        };
        let { _, instance } = await WebAssembly.instantiate(output_wasm_buffer, importObject);
        instance.exports.main();

        var wasm_memory = new Uint32Array(instance.exports.mem.buffer)
        // console.log(wasm_memory.slice(0, 200))

        if (!has_error) {
            observableStateStore.addConsoleMessage("✔ Execution finished.");
        }

        async function print_wat(buffer) {
            let w = await wabt()
            let module = w.readWasm(buffer, { readDebugNames: true });
            module.generateNames();
            module.applyNames();
            let wat = module.toText({
                foldExprs: true,
                inlineExport: false
            });
            observableStateStore.printConsoleWasm(wat);
        }

        process_export(output_image_info, wasm_memory, output_material_info);

        if (!has_error) {
            observableStateStore.addConsoleMessage("✔ Export finished.");
        }
    }

    function process_export(output_image_info, wasm_memory, output_material_info) {
        // single images
        let export_info = {}
        for (let [name, id] of Object.entries(output_image_info)) {
            export_info[name] = id;
        }
        export_images("textures", compiler.library_export(export_info));

        // materials 
        for (let [offset, names] of Object.entries(output_material_info)) {
            export_info = {}
            if (wasm_memory[offset] < 2147483647) {
                export_info[names[0] + "_" + names[1]] = wasm_memory[offset];
            }
            export_images(names[0], compiler.library_export(export_info));
        }
    }

    function export_images(category, result_images) {
        observableStateStore.clearOutputImage();
        for (let [name, data] of Object.entries(result_images)) {
            let image = {
                src: image_to_src(data.width, data.height, data.pixels),
                width: data.width,
                height: data.height,
            };
            observableStateStore.addOutputImage(category, name, image);
        }
    }

    function image_to_src(width, height, pixels) {
        let canvas = document.createElement("canvas");
        canvas.width = width;
        canvas.height = height;
        let context = canvas.getContext("2d");
        let imageData = context.createImageData(width, height);
        imageData.data.set(pixels);
        context.putImageData(imageData, 0, 0);
        return canvas.toDataURL()
    }
}

main();





