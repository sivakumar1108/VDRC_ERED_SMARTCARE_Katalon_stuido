import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//GlobalVariable.j4u_Apioffer_random_msisdn = '123411111'
Response = WS.sendRequest(findTestObject('004_J4U_Apioffers_automation/02_Random Normal/j4u-Normal Random voice'), FailureHandling.CONTINUE_ON_FAILURE)

offer1 = WS.getElementPropertyValue(Response, 'Offers[0].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer2 = WS.getElementPropertyValue(Response, 'Offers[1].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

offer3 = WS.getElementPropertyValue(Response, 'Offers[2].OfferID', FailureHandling.CONTINUE_ON_FAILURE)

def list = ['120003', '120002', '120001', '611143', '611142', '611141', '611140', '611139', '611138', '611137', '611136'
    , '611135', '611134', '611048', '611047', '611046', '611045', '611044', '611043', '611042', '611041', '611040', '611039'
    , '611038', '611037', '611036', '611035', '611034', '611033', '611032', '611031', '611030', '611029', '611152', '611151'
    , '611150', '611149', '611148', '611147', '611146', '611145', '611144', '611133', '611132', '611131', '611130', '611129'
    , '611128', '611127', '611126', '611125', '611124', '611123', '611122', '611121', '611120', '611119', '611258', '611266'
    , '611259', '611267', '611268', '611263', '611256', '611264', '611057', '611056', '611055', '611054', '611053', '611052'
    , '611051', '611050', '611049', '611257', '611265', '611272', '611273', '611260', '611274', '611261', '611269', '611262'
    , '611270', '611271', '611238', '611246', '611239', '611247', '611248', '611243', '611236', '611244', '611253', '611245'
    , '611237', '611254', '611255', '611240', '611241', '611249', '611242', '611251', '611252']

println(offer1)

println(offer2)

println(offer3)

assert offer1 in list

assert offer2 in list

assert offer3 in list

WS.comment('Normal Random Voice offers flow is working fine.')

