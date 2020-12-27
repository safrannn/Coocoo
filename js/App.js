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
import CreateIcon from '@material-ui/icons/Create';

import './App.css';

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
                <IconButton >
                    <PlayCircleFilledIcon />
                </IconButton>
            </Grid>
        </Grid>

    );
}

function CodeText() {
    const classes = useStyles();
    const [value, setValue] = React.useState('Controlled');

    const handleChange = (event) => {
        setValue(event.target.value);
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
                />
            </form>
        </Grid>
    );
}

function CodeConsole() {
    const classes = useStyles();
    const [value, setValue] = React.useState(0);

    const handleChange = (event, newValue) => {
        setValue(newValue);
    };
    return (
        <Grid container direction="column">
            <Tabs value={value} onChange={handleChange} aria-label="code_console">
                <Tab icon={<BugReportIcon />} {...a11yProps(0)} className={classes.codeConsoleTab} />
                <Tab icon={<NotesIcon />} {...a11yProps(1)} className={classes.codeConsoleTab} />
            </Tabs>
            <Divider />
            <TabPanel value={value} index={0} >
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
            </TabPanel>
            <TabPanel value={value} index={1}>
                <form className={classes.codeText} noValidate autoComplete="off">
                    <InputBase
                        id="console"
                        multiline
                        rowsMax={11}
                        readOnly
                        value="wasm output"
                        className={classes.codeText}
                    />
                </form>
            </TabPanel>
        </Grid >
    );
}

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
    return (
        <Grid container className={classes.bar}>
            <input accept="image/*" className={classes.inputIcon} id="image_upload" type="file" />
            <label htmlFor="image_upload" >
                <IconButton component="span" id="image_upload_icon">
                    <PublishIcon />
                </IconButton>
            </label>
            <IconButton>
                <DeleteIcon />
            </IconButton>
        </Grid>
    );
}


let ImageTile = (props) => {
    const { tile } = props;
    const classes = useStyles();

    const [value, setValue] = React.useState(tile.title);
    const handleChange = (event) => {
        setValue(event.target.value);
        STATE["EFG"] = "ABC";
    };

    return (
        <GridListTile key={tile.title} cols={2} >
            <img src={tile.img} alt={tile.title} />
            <GridListTileBar
                className={classes.imageTitleBar}
                title={
                    <form noValidate autoComplete="off">
                        <InputBase value={value} id={tile.title} onChange={handleChange}>

                        </InputBase>
                    </form>
                }
            // actionIcon={}
            />
        </GridListTile >
    )
};


function ImageInputPanel() {
    const classes = useStyles();
    return (
        <GridList cellHeight={150} className={classes.imageInputPanel} cols={6}>
            {imageInputData.map((tile) => ImageTile({ tile }))}
        </GridList>
    );
}

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

function ImageOutputPanel() {
    const classes = useStyles();
    return (
        <GridList cellHeight={150} className={classes.imageInputPanel} cols={6}>
            {imageInputData.map((tile) => ImageTile({ tile }))}
        </GridList>
    );
}




const imageInputData = [
    {
        id: 1,
        img: "https://www.gettyimages.com/gi-resources/images/500px/983794168.jpg",
        title: '11111111111111111111111111111111111111111111111111111111111111111',
        cols: 2,
    },
    {
        id: 2,
        img: "https://helpx.adobe.com/content/dam/help/en/stock/how-to/visual-reverse-image-search/jcr_content/main-pars/image/visual-reverse-image-search-v2_intro.jpg",
        title: '2',
        cols: 2,
    },
    {
        id: 3,
        img: "https://photojournal.jpl.nasa.gov/jpeg/PIA23689.jpg",
        title: '3',
        cols: 2,
    },
    {
        id: 4,
        img: "https://images.unsplash.com/photo-1494548162494-384bba4ab999?ixid=MXwxMjA3fDB8MHxzZWFyY2h8MXx8ZGF3bnxlbnwwfHwwfA%3D%3D&ixlib=rb-1.2.1&w=1000&q=80",
        title: '4',
        cols: 2,
    },
    {
        id: 5,
        img: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR0AnQUza0zwKklcP9bASAX5JRS_m08RouqWA&usqp=CAU",
        title: '5',
        cols: 2,
    },
    {
        id: 6,
        img: "https://www.w3schools.com/w3css/img_lights.jpg",
        title: '6',
        cols: 2,
    },
    {
        id: 7,
        img: "https://www.w3schools.com/w3css/img_forest.jpg",
        title: '7',
        cols: 2,
    }, {
        id: 8,
        img: "https://ichef.bbci.co.uk/news/976/cpsprodpb/1572B/production/_88615878_976x1024n0037151.jpg",
        title: '8',
        cols: 2,
    }
]

const imageOutputData = [
    {
        id: 9,
        img: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTgaNi3D4eNMnyGH56_1gD-0485xuI07N6ztw&usqp=CAU",
        title: '9',
        cols: 2,
    },
    {
        id: 10,
        img: "https://image.shutterstock.com/image-photo/large-beautiful-drops-transparent-rain-260nw-668593321.jpg",
        title: '10',
        cols: 2,
    }]


window.STATE = {
    imageInput: imageInputData,
    code: "",
    imageOutput: imageOutputData,
};
