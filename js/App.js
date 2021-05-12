import React, { Component, useEffect } from 'react';
import { toJS } from 'mobx'
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

import { observer } from "mobx-react";
import { makeObservable, observable, action } from "mobx"

import wabt from "wabt";

import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls"

import './App.css';


const regeneratorRuntime = require("regenerator-runtime");

class ObservableStateStore {
    imageInputFiles = new Map();
    imageOutputFiles = new Map();
    materialInstanceType = {};
    code = "";
    consoleMessage = "";
    consoleWasm = "";

    leftSiderTabValue = 0;
    codeConsoleTabValue = 0;
    imageBlockTabValue = 0;
    imageOutputFileListTabValue = 0;
    imageOutputFileListArray = [];

    scene = new THREE.Scene();
    mesh = new THREE.Mesh(
        new THREE.SphereGeometry(5, 32, 32),
        new THREE.MeshLambertMaterial({
            color: 0x000000,
        }));;
    camera = {};
    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });


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
            getImageOutputFilesSize: action,
            getOutputMaterialInstance: action,
            clearOutputImage: action,
            materialInstanceType: observable,
            addMaterialInstanceType: action,
            getMaterialInstanceType: action,
            changeCode: action,
            clearCode: action,
            printConsoleMessage: action,
            addConsoleMessage: action,
            clearConsoleMessage: action,
            printConsoleWasm: action,
            leftSiderTabValue: observable,
            codeConsoleTabValue: observable,
            imageBlockTabValue: observable,
            imageOutputFileListTabValue: observable,
            imageOutputFileListArray: observable,
            setImageOutputFileListArray: action,
            getImageOutputFileListArrayContent: action,
            changeImageOutputFileListTabValue: action,
            scene: observable,
            mesh: observable,
            camera: observable,
            renderer: observable,
            initThreeJS: action,
            updateMesh: action,
            resetMesh: action,
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

    addOutputImage(material_instance_name, image_name, image_data) {
        if (!this.imageOutputFiles.get(material_instance_name)) {
            this.imageOutputFiles.set(material_instance_name, {});
        }
        this.imageOutputFiles.get(material_instance_name)[image_name] = image_data;
    }

    renameOutputImage(material_instance_name, oldName, newName) {
        if (!this.imageOutputFiles.get(material_instance_name).hasOwnProperty(newName)) {
            return false
        } else {
            var image_data = this.imageOutputFiles.get(material_instance_name)[oldName];
            delete this.imageOutputFiles.get(material_instance_name)[oldName];
            this.imageOutputFiles.get(material_instance_name)[newName] = image_data;
            return true
        }
    }
    getImageOutputFilesSize() {
        return this.imageOutputFiles.size;
    }

    getOutputMaterialInstance(material_instance) {
        return this.imageOutputFiles.get(material_instance)
    }

    clearOutputImage() {
        this.imageOutputFiles.clear();
        this.materialInstanceType = {};
    }

    addMaterialInstanceType(material_instance_name, material_type) {
        if (this.materialInstanceType[material_instance_name] == undefined) {
            this.materialInstanceType[material_instance_name] = material_type;
        }
    }

    getMaterialInstanceType(material_instance_name) {
        return this.materialInstanceType[material_instance_name];
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

    changeImageOutputFileListTabValue(newValue) {
        this.imageOutputFileListTabValue = newValue;
    }

    setImageOutputFileListArray(newValue) {
        this.imageOutputFileListArray = newValue;
    }

    getImageOutputFileListArrayContent(index) {
        return this.imageOutputFileListArray[index];
    }

    initThreeJS(container) {

        var width = window.innerWidth * 5 / 12;
        var height = window.innerHeight * 0.4;

        this.renderer.setClearColor("#000000");
        this.renderer.setSize(width, height);
        container.appendChild(this.renderer.domElement);

        this.camera = new THREE.PerspectiveCamera(55, width / height, 0.01, 1000);
        this.camera.position.set(-10, 12, 15)
        this.camera.lookAt(0, 0, 0)

        // var geometry = new THREE.BoxGeometry(5, 5, 5);
        this.mesh.name = "mesh";
        this.scene.add(this.mesh);

        const controls = new OrbitControls(this.camera, this.renderer.domElement);
        controls.dampingFactor = 0.4;
        controls.enableDamping = true;

        var spotLight1 = new THREE.SpotLight(0xffffff, 1.0)
        spotLight1.position.set(-100, 200, 50);
        spotLight1.castShadow = true;
        this.scene.add(spotLight1);
        var spotLight2 = new THREE.SpotLight(0xffffff, 0.4)
        spotLight2.position.set(100, -200, -50);
        spotLight2.castShadow = false;
        this.scene.add(spotLight2);
        var aolight = new THREE.AmbientLight(0xffffff, 0.2);
        this.scene.add(aolight);
    }

    updateMesh(material_instance_name) {
        this.scene.remove(this.scene.getObjectByName("mesh"));

        var geometry = new THREE.SphereGeometry(5, 32, 32);
        var material = new THREE.MeshLambertMaterial({
            color: 0x000000,
        });

        var material_maps = toJS(observableStateStore.getOutputMaterialInstance(material_instance_name))
        var material_instance_type = this.getMaterialInstanceType(material_instance_name);
        if (material_instance_type == "PBRMetalness") {
            // missing cavity and subsurface scattering
            material = new THREE.MeshStandardMaterial();
            if ("diffuse" in material_maps) {
                material.map = new THREE.TextureLoader().load(material_maps["diffuse"].src);
            }
            if ("metalness" in material_maps) {
                material.metalnessMap = new THREE.TextureLoader().load(material_maps["metalness"].src);
            }
            if ("normal" in material_maps) {
                material.normalMap = new THREE.TextureLoader().load(material_maps["normal"].src);
            }
            if ("transparency" in material_maps) {
                material.alphaMap = new THREE.TextureLoader().load(material_maps["transparency"].src);
            }
            if ("roughness" in material_maps) {
                material.roughnessMap = new THREE.TextureLoader().load(material_maps["roughness"].src);
            }
            if ("ao" in material_maps) {
                material.aoMap = new THREE.TextureLoader().load(material_maps["ao"].src);
            }
            if ("displacement" in material_maps) {
                material.displacementMap = new THREE.TextureLoader().load(material_maps["displacement"].src);
            }
            if ("emissive" in material_maps) {
                material.emissiveMap = new THREE.TextureLoader().load(material_maps["emissive"].src);
            }
        } else if (material_instance_type == "PBRSpecular") {
            // missing specular workflow in threejs
            // missing specular, glossiness,cavity and subsurface scattering
            material = new THREE.MeshStandardMaterial();
            if ("albedo" in material_maps) {
                material.map = new THREE.TextureLoader().load(material_maps["albedo"].src);
            }
            if ("normal" in material_maps) {
                material.normalMap = new THREE.TextureLoader().load(material_maps["normal"].src);
            }
            if ("transparency" in material_maps) {
                material.alphaMap = new THREE.TextureLoader().load(material_maps["transparency"].src);
            }
            if ("ao" in material_maps) {
                material.aoMap = new THREE.TextureLoader().load(material_maps["ao"].src);
            }
            if ("displacement" in material_maps) {
                material.displacementMap = new THREE.TextureLoader().load(material_maps["displacement"].src);
            }
            if ("emissive" in material_maps) {
                material.emissiveMap = new THREE.TextureLoader().load(material_maps["emissive"].src);
            }
        } else if (material_instance_type == "UnityStandardSpecular") {
            // missing specular workflow in threejs
            // missing specular and detailed_mask
            material = new THREE.MeshStandardMaterial();
            if ("albedo" in material_maps) {
                material.map = new THREE.TextureLoader().load(material_maps["albedo"].src);
            }
            if ("normal" in material_maps) {
                material.normalMap = new THREE.TextureLoader().load(material_maps["normal"].src);
            }
            if ("transparency" in material_maps) {
                material.alphaMap = new THREE.TextureLoader().load(material_maps["transparency"].src);
            }
            if ("ao" in material_maps) {
                material.aoMap = new THREE.TextureLoader().load(material_maps["ao"].src);
            }
            if ("height" in material_maps) {
                material.displacementMap = new THREE.TextureLoader().load(material_maps["height"].src);
            }
            if ("emissive" in material_maps) {
                material.emissiveMap = new THREE.TextureLoader().load(material_maps["emissive"].src);
            }
        }

        this.mesh = new THREE.Mesh(geometry, material);
        this.mesh.name = "mesh";
        this.scene.add(this.mesh);
    }

    resetMesh() {
        this.scene.remove(this.scene.getObjectByName("mesh"));
        this.mesh = new THREE.Mesh(
            new THREE.SphereGeometry(5, 32, 32),
            new THREE.MeshLambertMaterial({
                color: 0x000000,
            }));
        this.mesh.name = "mesh";
        this.scene.add(this.mesh);
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
        padding: theme.spacing(1),
        flexGrow: 1,
    },
    codeBlock: {
        // height: "calc(100vh - 45px)",
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
        width: "100%",
        flexGrow: 1,
        display: "flex",
        height: "calc(2*(100vh - 45px - 48px - 48px)/5 - 1)",
        padding: theme.spacing(1),
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
    imageInputTab: {
        display: 'flex',
        flexWrap: 'wrap',
        overflow: 'scroll',
    },
    imageInputPanel: {
        padding: theme.spacing(1),
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
        flexGrow: 1,
        display: 'flex',
        height: 'calc(60vh - 93px)',
        width: '100%'
    },
    imageOutputFileListTabs: {
        borderRight: `1px solid ${theme.palette.divider}`,
    },
    imageOutputFileListPanel: {
        width: '75%',
        maxWidth: '75%',
    },
}));

// var console_log = console.log;
// console.log = function (message) {
//     observableStateStore.addConsoleMessage(message);
//     console_log.apply(console, arguments);
// };

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
                <Box>
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

const LeftSider = observer(() => {
    const classes = useStyles();

    const handleChange = (event, newValue) => {
        observableStateStore.leftSiderTabValue = newValue;
    };

    return (
        <div className={classes.leftSider}>
            <Tabs
                orientation="vertical"
                value={observableStateStore.leftSiderTabValue}
                onChange={handleChange}
                className={classes.leftSiderTabs}
            >
                <Tab icon={<MenuBookIcon />} {...a11yProps("leftSider-tabs", 0)} className={classes.leftSiderTab} />
            </Tabs>
            <TabPanel value={observableStateStore.leftSiderTabValue} index={0} prefix="leftSider-tabs">
                <LeftSiderDocument></LeftSiderDocument>
            </TabPanel>
        </div>
    );
})

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

const CodeConsole = observer(() => {
    const classes = useStyles();
    const handleChange = (event, newValue) => {
        observableStateStore.codeConsoleTabValue = newValue;
    };

    return (
        <Grid>
            <Tabs value={observableStateStore.codeConsoleTabValue} onChange={handleChange} aria-label="code_console">
                <Tab icon={<NotesIcon />} {...a11yProps("codeConsole-tabs", 0)} />
                <Tab icon={<BugReportIcon />} {...a11yProps("codeConsole-tabs", 1)} />
            </Tabs>
            <Divider />
            <TabPanel value={observableStateStore.codeConsoleTabValue} index={0} prefix="codeConsole-tabs">
                <CodeConsoleMessage />
            </TabPanel>
            <TabPanel value={observableStateStore.codeConsoleTabValue} index={1} prefix="codeConsole-tabs">
                <CodeConsoleWasm />
            </TabPanel>
        </Grid >
    );
})

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


const ImageBlock = observer(() => {
    const classes = useStyles();
    const handleChange = (event, newValue) => {
        observableStateStore.imageBlockTabValue = newValue;
        observableStateStore.changeImageOutputFileListTabValue(0)
    };

    return (
        <Grid container className={classes.imageBlock}>
            <div id="loadingSpinner" className="loader loader-default" data-text="Loading"></div>
            <Grid container direction="column">
                <Tabs variant="fullWidth" value={observableStateStore.imageBlockTabValue} onChange={handleChange} aria-label="image block">
                    <Tab label="Input" {...a11yProps("imageBlock-tabs", 0)} />
                    <Tab label="Output" {...a11yProps("imageBlock-tabs", 1)} />
                </Tabs>
                <Divider />
                <TabPanel value={observableStateStore.imageBlockTabValue} index={0} prefix="imageBlock-tabs">
                    <Grid container direction="column">
                        <Grid item align="center" xs={12}>
                            <ImageUpload />
                        </Grid>
                        <Grid item xs={12}>
                            <ImageInputTab />
                        </Grid>
                    </Grid>
                </TabPanel>
                <TabPanel value={observableStateStore.imageBlockTabValue} index={1} prefix="imageBlock-tabs">
                    <ImageOutputTab />
                </TabPanel>
            </Grid>
        </Grid >
    );
})

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
                };
                let reader = new FileReader();
                reader.onload = function () {
                    image.src = reader.result;
                    observableStateStore.addInputImage(imageName, image);
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
    const classes = useStyles();
    return (
        <div>
            <ImageOutputFileList />
            <ImageOutputRender />
        </div>
    );
})

const ImageOutputFileList = observer(() => {
    const classes = useStyles();

    const handleChange = (event, newTabValue) => {
        observableStateStore.changeImageOutputFileListTabValue(newTabValue)
        var material_name = observableStateStore.getImageOutputFileListArrayContent(newTabValue);
        observableStateStore.updateMesh(material_name);
        renderMaterial();
    };

    const tab = [];
    const tabPanel = [];
    const imageOutputFileListArray = [];

    let i = 0;
    for (var [material_instance, info] of observableStateStore.imageOutputFiles) {
        tab.push(
            <Tab key={i} label={material_instance} {...a11yProps("imageOutputFileList-tabs", i)} />
        )
        tabPanel.push(
            <TabPanel value={observableStateStore.imageOutputFileListTabValue}
                key={i}
                index={i}
                prefix="imageOutputFileList-tabs"
                className={classes.imageOutputFileListPanel}
            >
                <ImageOutputImageFileDisplay material_instance={material_instance} />
            </TabPanel>
        )
        imageOutputFileListArray.push(material_instance);
        i += 1;
    }

    observableStateStore.setImageOutputFileListArray(imageOutputFileListArray);

    return (
        <div className={classes.imageOutputFileList} >
            <Tabs
                orientation="vertical"
                scrollButtons="auto"
                value={observableStateStore.imageOutputFileListTabValue}
                onChange={handleChange}
                className={classes.imageOutputFileListTabs}
            >
                {tab}
            </Tabs>
            {tabPanel}
        </div>
    );
})

const ImageOutputImageFileDisplay = observer(({ material_instance }) => {
    const classes = useStyles();

    let files = observableStateStore.getOutputMaterialInstance(material_instance);
    if (files == undefined) {
        console.log(material_instance, " doesn't exist.")
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
                        <Button href={tile.src} download={material_instance + "_" + tileName}>
                            <GetAppIcon>
                            </GetAppIcon>
                        </Button>
                    }
                />
            </GridListTile >
        )
    }
    return (
        <GridList cellHeight={150} cols={6}>
            {items}
        </GridList>
    );
})

const ImageOutputRender = observer(() => {
    useEffect(() => {
        var container = document.getElementById('render_container');

        observableStateStore.initThreeJS(container);

        renderMaterial();
    }, [])

    return (
        <div id="render_container">
        </div>
    );
})

const renderMaterial = function () {
    var scene = observableStateStore.scene;
    var camera = observableStateStore.camera;
    var mesh = observableStateStore.mesh;
    var renderer = observableStateStore.renderer;
    const animate = function () {
        mesh.rotation.y -= 0.002;

        requestAnimationFrame(animate);
        renderer.render(scene, camera);
    };
    animate();
}

function pixelsToUrl(width, height, pixels) {
    let canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    let context = canvas.getContext("2d");
    let imageData = context.createImageData(width, height);
    imageData.data.set(pixels);
    context.putImageData(imageData, 0, 0);
    return canvas.toDataURL()
}

function urlToImage(url) {
    let image = {
        width: 0,
        height: 0,
        pixels: [],
    };

    let img = new Image();
    img.src = url;
    let canvas = document.createElement("canvas");
    canvas.width = img.width;
    canvas.height = img.height;
    let context = canvas.getContext("2d");
    context.drawImage(img, 0, 0, img.width, img.height);
    image.pixels = context.getImageData(0, 0, img.width, img.height).data;
    image.width = img.width;
    image.height = img.height;
    canvas.remove();
    return image
}



async function main() {
    let compiler = await import("../pkg/compiler.js");

    function processImageInput() {
        let names = [...observableStateStore.imageInputFiles.keys()];
        for (const [name, data] of observableStateStore.imageInputFiles) {
            let image = urlToImage(data.src)
            compiler.library_add_image(name, image.width, image.height, image.pixels);
        }
        return names;
    }

    document.getElementById('run').onclick = async function () {
        var loadElement = document.getElementById("loadingSpinner");
        loadElement.classList.add("is-active");

        observableStateStore.resetMesh();

        compiler.library_reset();
        observableStateStore.clearConsoleMessage();
        observableStateStore.clearOutputImage();

        let image_names = processImageInput();
        observableStateStore.addConsoleMessage(time_now() + " ✔ Image uploaded.");

        let output = compiler.code_to_wasm(observableStateStore.code, image_names);
        let output_wasm_buffer = new Uint8Array(output[0]);
        let output_textures_info = output[1]; // image_name, image_id
        let output_materials_info = output[2]; // position in mem, [material_name, channel_name, material_type]
        // console.log(output)
        // console.log("output_materials_info", output_materials_info)

        observableStateStore.addConsoleMessage(time_now() + " ✔ Compile finished.");

        print_wat(output_wasm_buffer);

        let wasmImportObject = {
            env: {
                logger: function (arg) {
                    console.log(arg);
                },
                resize: function (img_id, width, height) {
                    return compiler.resize(img_id, width, height)
                },
                darken: function (img_id, value) {
                    return compiler.darken(img_id, value)
                },
                blank_image: function (r, g, b, a, width, height) {
                    return compiler.blank_image(r, g, b, a, width, height)
                },
                grayscale: function (img_id) {
                    return compiler.grayscale(img_id)
                },
                invert_color: function (img_id) {
                    return compiler.invert(img_id)
                },
                flip_horizontal: function (img_id) {
                    return compiler.flip_horizontal(img_id)
                },
                flip_vertical: function (img_id) {
                    return compiler.flip_vertical(img_id)
                },
                mask_channel_r: function (img_id) {
                    return compiler.mask_channel_r(img_id)
                },
                mask_channel_g: function (img_id) {
                    return compiler.mask_channel_g(img_id)
                },
                mask_channel_b: function (img_id) {
                    return compiler.mask_channel_b(img_id)
                },
            }
        };
        let { _, instance } = await WebAssembly.instantiate(output_wasm_buffer, wasmImportObject);
        observableStateStore.addConsoleMessage(time_now() + " ✔ Wasm module instantiated.");

        instance.exports.main();
        // console.log(wasm_memory.slice(0, 200))
        observableStateStore.addConsoleMessage(time_now() + " ✔ Wasm module executed.");

        var wasm_memory = new Uint32Array(instance.exports.mem.buffer)
        process_export(output_textures_info, wasm_memory, output_materials_info);

        observableStateStore.addConsoleMessage(time_now() + " ✔ Export finished.");

        loadElement.classList.remove("is-active");

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
    }

    function process_export(output_textures_info, wasm_memory, output_materials_info) {
        // single images
        let export_info = {}
        for (let [name, id] of Object.entries(output_textures_info)) {
            export_info[name] = id;
        }
        export_images("textures", compiler.library_export(export_info), "");

        // materials
        for (let [offset, names] of Object.entries(output_materials_info)) {
            observableStateStore.addMaterialInstanceType(names[0], names[2]);
            if (wasm_memory[offset] < 2147483647) {
                export_info = {}
                export_info[names[1]] = wasm_memory[offset];
                export_images(names[0], compiler.library_export(export_info));
            }
        }
    }

    function export_images(material_instance_name, received_images) {
        for (let [image_name, data] of Object.entries(received_images)) {
            let image = {
                src: pixelsToUrl(data.width, data.height, data.pixels),
                width: data.width,
                height: data.height,
            };
            observableStateStore.addOutputImage(material_instance_name, image_name, image);
        }
    }
}

function time_now() {
    var new_date = new Date();
    return new_date.toLocaleTimeString('en-US', { hour12: false }) + ":" + new_date.getUTCMilliseconds();
}
main();





