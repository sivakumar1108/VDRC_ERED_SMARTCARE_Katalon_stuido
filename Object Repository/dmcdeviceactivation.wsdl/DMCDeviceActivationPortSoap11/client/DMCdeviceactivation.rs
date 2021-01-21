<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DMCdeviceactivation</name>
   <tag></tag>
   <elementGuidId>aedea84d-ec4a-429b-a4ca-4f81bba81a41</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:com=&quot;http://schema.concierge.com&quot;>
&lt;soapenv:Header/>
&lt;soapenv:Body>
&lt;com:clientRequest>
&lt;EaiEnvelope xmlns:ser=&quot;http://schema.concierge.com/Services&quot;>
&lt;ApplicationName>MTNZ&lt;/ApplicationName>
&lt;Domain>abl_Portal&lt;/Domain>&lt;Service>Services&lt;/Service>&lt;Language>En&lt;/Language>&lt;UserId>admin&lt;/UserId>&lt;Sender>admin&lt;/Sender>&lt;MessageId>1&lt;/MessageId>&lt;Payload>&lt;ser:Services>&lt;ser:Request>&lt;ser:Operation_Name>abillityReferenceApi&lt;/ser:Operation_Name>&lt;ser:ChangeServicesRequest>&lt;ser:request>&lt;EVENT xmlns=&quot;&quot;>&lt;REQUEST API_CODE=&quot;7623&quot; EXTERNAL_USER=&quot;admin&quot; EXTERNAL_APPLICATION=&quot;admin&quot; EXTERNAL_REFERENCE=&quot;admin&quot; CLIENT_ID=&quot;ZM&quot; OPERATION_NAME=&quot;ServiceActivation&quot; EXTERNAL_SYSTEMS_LOG_REFERNCE=&quot;20201202142213&quot; ENTITY_ID=&quot;+243987654321&quot; REQUESTED_APPLICATION=&quot;TPS&quot; SERVICE_REQUEST_TYPE=&quot;N&quot; BUNDLE_SUB_CATEGORY=&quot;&quot; SUBSCRIPTION_FLAG=&quot;0&quot; EQUIP_TYPE=&quot;IMEI&quot; EQUIP_NUM=&quot;351578100496827&quot; INFO_LEVEL=&quot;1&quot;/>&lt;/EVENT>&lt;/ser:request>&lt;/ser:ChangeServicesRequest>&lt;/ser:Request>&lt;/ser:Services>&lt;/Payload>&lt;/EaiEnvelope>&lt;/com:clientRequest>&lt;/soapenv:Body>&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${decdeviceactivation_Url}</soapServiceEndpoint>
   <soapServiceFunction>client</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.decdeviceactivation_Url</defaultValue>
      <description></description>
      <id>0624c001-907b-4db8-b082-b85b5d16f0d6</id>
      <masked>false</masked>
      <name>decdeviceactivation_Url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>dmcdeviceactivation.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
